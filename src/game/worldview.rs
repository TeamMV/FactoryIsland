use crate::game::{place_tile, Game};
use crate::gameloop::FactoryIslandClient;
use crate::gamesettings::GameSettings;
use crate::{drawutils, input};
use crate::player::ClientPlayer;
use crate::rendering::WorldShaders;
use crate::ui::display::chat::Chat;
use crate::ui::display::TileSelection;
use crate::world::ClientWorld;
use api::server::packets::common::{ClientDataPacket, ServerStatePacket, TileKind};
use api::server::packets::world::TileSetFromClientPacket;
use api::server::{ClientBoundPacket, ServerBoundPacket};
use api::world::tiles::pos::TilePos;
use api::world::tiles::Orientation;
use mvengine::input::consts::MouseButton;
use mvengine::math::vec::Vec2;
use mvengine::modify_style;
use mvengine::net::server::ClientId;
use mvengine::rendering::pipeline::RenderingPipeline;
use mvengine::rendering::{OpenGLRenderer, RenderContext, CLEAR_FLAG};
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::elements::prelude::*;
use mvengine::ui::elements::{Element, UiElementStub};
use mvengine::ui::styles::InheritSupplier;
use mvengine::window::Window;
use mvengine_proc::style_expr;
use mvengine_proc::ui;
use mvutils::hashers::U64IdentityHasher;
use mvutils::thread::ThreadSafe;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use log::trace;
use mvengine::ui::context::UiResources;
use mvengine::ui::rendering::adaptive::AdaptiveFill;
use mvengine::ui::rendering::WideRenderContext;
use api::ingredients::IngredientKind;
use api::world::SingleTileUnit;
use crate::drawutils::Fill;
use crate::input::{ESCAPE, ROTATE_L, ROTATE_R};
use crate::res::R;
use crate::ui::manager::{GameUiManager, UI_ESCAPE_SCREEN};
use crate::world::tiles::impls::CLIENT_TILE_REG;

pub type RP = RenderingPipeline<OpenGLRenderer>;

/// This hold everything only present when the player is inside a world.
pub struct WorldView {
    //ui
    click_area: ThreadSafe<Element>,
    pub tile_selection: TileSelection,
    pub ingredients: Vec<IngredientKind>,
    pub chat: Chat,

    //game
    pub world: ClientWorld,
    pub tile_size: i32,
    pub player: ClientPlayer,
    pub other_players: HashMap<ClientId, ClientPlayer, U64IdentityHasher>,
    pub orientation: Orientation,

    //rendering
    pub world_pipeline: RP,
    pub player_pipeline: RP,
    pub overlay_pipeline: RP,
    pub frame: u64,
    initialized: bool,
}

impl WorldView {
    pub fn new(window: &mut Window, server_state_packet: ServerStatePacket, game: &Game) -> Self {
        let click_area = ui! {
            <Ui context={window.ui().context()}>
                <Div id="click_area" style="padding: none; margin: none; position: absolute; x: 0; y: 0; width: 100%; height: 100%; background.resource: none; border.resource: none;"/>
            </Ui>
        };

        let mut map = HashMap::with_hasher(U64IdentityHasher::default());
        for entry in server_state_packet.players {
            let player = ClientPlayer::new(window.width(), window.height(), entry.data);
            map.insert(entry.client_id, player);
        }

        let shaders = WorldShaders::new();

        //Unwrap here cuz what are ya gonna do without rendering
        let mut world_pipeline = RenderingPipeline::new_default_opengl(window).unwrap();
        world_pipeline.create_post(window);
        //for custom blend shader
        world_pipeline.use_custom_backbuffer(window);
        let [
            ssao,
            clouds,
            overlay,
            overlay_blend
        ] = [
            shaders.ssao,
            shaders.clouds,
            shaders.overlay,
            shaders.overlay_blend,
        ];
        world_pipeline.add_post_step(ssao);
        world_pipeline.add_post_step(clouds);

        let player_pipeline = RenderingPipeline::new_default_opengl(window).unwrap();
        let mut overlay_pipeline = RenderingPipeline::new_default_opengl(window).unwrap();
        overlay_pipeline.create_post(window);
        overlay_pipeline.add_post_step(overlay);
        overlay_pipeline.use_custom_blend_shader(overlay_blend);

        let mut this = Self {
            click_area: ThreadSafe::new(click_area),
            tile_selection: TileSelection::new(window, server_state_packet.tiles.into_iter()),
            ingredients: server_state_packet.ingredients,
            chat: Chat::new(window),
            world: ClientWorld::new(),
            tile_size: 50,
            player: ClientPlayer::new(1, 1, ClientDataPacket {
                profile: game.profile.clone(),
                render_distance: 1,
                client_id: server_state_packet.client_id,
            }),
            other_players: map,
            orientation: Orientation::North,
            world_pipeline,
            player_pipeline,
            overlay_pipeline,
            frame: 0,
            initialized: false,
        };

        this.resize(window);

        this
    }

    pub fn open(&mut self, window: &mut Window) {
        self.tile_selection.open(window, self.click_area.as_ref().clone());
        window.ui_mut().add_root(self.click_area.as_ref().clone());
    }

    pub fn close(&mut self, window: &mut Window) {
        window.ui_mut().remove_root(self.click_area.as_ref().clone());
    }

    pub fn resize(&mut self, window: &Window) {
        let w = window.info().width;
        let h = window.info().height;
        self.player.resize_view(w, h);
        for player in self.other_players.values_mut() {
            player.resize_view(w, h);
        }
        self.world_pipeline.resize(window);
        self.player_pipeline.resize(window);
        self.overlay_pipeline.resize(window);
    }

    pub fn player_join(&mut self, player: ClientPlayer, id: ClientId) {
        self.other_players.insert(id, player);
    }

    pub fn player_leave(&mut self, id: ClientId) {
        self.other_players.remove(&id);
    }

    pub fn draw(&mut self, window: &mut Window, prev_pipeline: Option<&mut RP>, next_pipeline: Option<&mut RP>, settings: &GameSettings) {
        if let Some(pl) = prev_pipeline {
            pl.next_pipeline(&mut self.world_pipeline);
        } else {
            self.world_pipeline.begin_frame();
        }

        //Important so the post shaders have depth values to work with
        OpenGLRenderer::enable_depth_test();

        trace!("Beginning of draw");
        self.world.draw(&mut self.world_pipeline, &self.player.camera.view_area, self.tile_size);

        //OpenGLRenderer::enable_depth_buffer();
        //draw raw world
        self.world_pipeline.advance(window, |_| {});

        //draw ssao
        if *settings.ssao_shader.read() {
            self.world_pipeline.advance(window, |_| {});
        } else {
            self.world_pipeline.skip();
        }

        //draw clouds
        if *settings.cloud_shader.read() {
            let cam_pos = Vec2::from_i32s(self.player.camera.pos);
            self.world_pipeline.advance(window, |s| {
                s.uniform_1f("FRAME", self.frame as f32);
                s.uniform_2fv("CAM", &cam_pos);
            });
        } else {
            self.world_pipeline.skip();
        }

        //Otherwise the ui will be behind the world LMAO
        OpenGLRenderer::disable_depth_test();

        self.world_pipeline.next_pipeline(&mut self.player_pipeline);

        self.draw_players();
        self.player_pipeline.advance(window, |_| {});


        if let Some(_) = self.tile_selection.selected_tile() {
            place_tile::draw_overlay(self, window, settings);

            if let Some(next) = next_pipeline {
                self.overlay_pipeline.next_pipeline(next);
            } else {
                self.overlay_pipeline.flush();
            }
        } else {
            if let Some(next) = next_pipeline {
                self.player_pipeline.next_pipeline(next);
            } else {
                self.player_pipeline.flush();
            }
        }
    }

    pub fn draw_players(&mut self) {
        for player in self.other_players.values() {
            player.draw_from_other_pov(&mut self.player_pipeline, &self.player.camera.view_area, self.tile_size);
        }

        self.player.draw(&mut self.player_pipeline, self.tile_size);
    }

    pub fn on_frame(&mut self, window: &mut Window, client: &mut FactoryIslandClient, ui_manager: &mut GameUiManager) {
        if !self.initialized {
            self.initialized = true;

            client.send(ServerBoundPacket::ClientData(self.player.data.clone()));
        }
        if window.input.was_action(ESCAPE) {
            ui_manager.goto(UI_ESCAPE_SCREEN, window);
        }
        let mut has_moved = false;
        let speed = self.player.speed * window.get_delta_t();
        if !self.chat.open {
            if window.input.is_action(input::MOVE_FORWARD) {
                self.player.move_by((0.0, speed), self.tile_size);
                has_moved = true;
            }
            if window.input.is_action(input::MOVE_BACK) {
                self.player.move_by((0.0, -speed), self.tile_size);
                has_moved = true;
            }
            if window.input.is_action(input::MOVE_LEFT) {
                self.player.move_by((-speed, 0.0), self.tile_size);
                has_moved = true;
            }
            if window.input.is_action(input::MOVE_RIGHT) {
                self.player.move_by((speed, 0.0), self.tile_size);
                has_moved = true;
            }
        }
        if window.input.was_action(input::CHAT) {
            self.chat.toggle(window, client);
        }
        if window.input.was_action(input::RELOAD_CHUNKS) {
            self.world.drop_all();
            client.send(ServerBoundPacket::RequestReload);
        }

        if has_moved {
            self.player.broadcast_position(client);
        }

        self.tile_selection.check_events();

        self.frame = self.frame.wrapping_add(1);

        //tile set
        if let Some(event) = &self.click_area.get().state().events.click_event {
            if event.button == MouseButton::Left && event.base.action == UiClickAction::Click {
                if let Some(tile) = self.tile_selection.selected_tile() {
                    let pos = TilePos::from_screen((window.input.mouse_x, window.input.mouse_y), &self.player.camera.view_area, self.tile_size);
                    if pos.distance_from(&self.player) <= self.player.reach {
                        client.send(ServerBoundPacket::TileSet(TileSetFromClientPacket {
                            pos,
                            tile_id: tile.id,
                            tile_state: vec![],
                            orientation: self.orientation,
                        }));
                    }
                } else {
                    let pos = TilePos::from_screen((window.input.mouse_x, window.input.mouse_y), &self.player.camera.view_area, self.tile_size);
                    if pos.distance_from(&self.player) <= self.player.reach {
                        client.send(ServerBoundPacket::TileSet(TileSetFromClientPacket {
                            pos,
                            tile_id: 0,
                            tile_state: vec![],
                            orientation: self.orientation,
                        }));
                    }
                }
            }
        }

        if window.input.was_action(ROTATE_L) {
            self.orientation = match self.orientation {
                Orientation::North => Orientation::West,
                Orientation::South => Orientation::East,
                Orientation::East => Orientation::North,
                Orientation::West => Orientation::South
            }
        } else if window.input.was_action(ROTATE_R) {
            self.orientation = match self.orientation {
                Orientation::North => Orientation::East,
                Orientation::South => Orientation::West,
                Orientation::East => Orientation::South,
                Orientation::West => Orientation::North
            }
        }
    }

    pub fn check_packet(&mut self, packet: ClientBoundPacket, game: &Game) {
        match packet {
            ClientBoundPacket::TileSet(packet) => {
                self.world.sync(packet, game);
            }
            ClientBoundPacket::ChunkData(packet) => {
                self.world.sync_chunk(packet, game);
            }
            ClientBoundPacket::PlayerMove(packet) => {
                self.player.move_to(packet.pos, self.tile_size);
            }
            ClientBoundPacket::OtherPlayerMove(packet) => {
                if let Some(player) = self.other_players.get_mut(&packet.client_id) {
                    player.move_to(packet.pos, self.tile_size);
                }
            }
            ClientBoundPacket::OtherPlayerJoin(packet) => {
                let id = packet.client_id;
                let player = ClientPlayer::new(self.player.camera.width, self.player.camera.height, packet.client_data);
                self.player_join(player, id);
            }
            ClientBoundPacket::OtherPlayerLeave(packet) => {
                self.player_leave(packet.client_id);
            }
            ClientBoundPacket::ChunkUnload(packet) => {
                self.world.drop_chunk(packet.pos);
            }
            ClientBoundPacket::OtherPlayerChat(packet) => {
                self.chat.push_message(packet);
            }
            ClientBoundPacket::PlayerDataPacket(packet) => {
                self.player.data_packet(packet, self.tile_size);
            }
            ClientBoundPacket::MultiTilePlacedPacket(packet) => {
                if let Some(chunk) = self.world.get_chunk_mut(packet.placement.pos.chunk_pos) {
                    chunk.multitiles.push(packet.placement.into());
                }
            }
            ClientBoundPacket::MultiTileDestroyedPacket(packet) => {
                if let Some(chunk) = self.world.get_chunk_mut(packet.chunk_pos) {
                    let mut remove = None;
                    for i in 0..chunk.multitiles.len() {
                        if chunk.multitiles[i].uuid == packet.placement_id {
                            remove = Some(i);
                            break;
                        }
                    }
                    if let Some(i) = remove {
                        chunk.multitiles.remove(i);
                    }
                }
            }
            _ => {}
        }
    }
}