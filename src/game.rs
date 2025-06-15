use crate::gameloop::FactoryIslandClient;
use crate::input;
use crate::mods::LocalModManager;
use crate::player::ClientPlayer;
use crate::ui::display::TileSelection;
use crate::world::ClientWorld;
use api::registry::Registry;
use api::server::packets::common::{ClientDataPacket, ServerStatePacket, TileKind};
use api::server::packets::world::TileSetFromClientPacket;
use api::server::ServerBoundPacket;
use api::world::tiles::pos::TilePos;
use api::world::tiles::terrain::TerrainTile;
use api::world::tiles::Orientation;
use bytebuffer::ByteBuffer;
use log::error;
use mvengine::input::consts::MouseButton;
use mvengine::input::Input;
use mvengine::modify_style;
use mvengine::net::server::ClientId;
use mvengine::rendering::RenderContext;
use mvengine::ui::elements::div::Div;
use mvengine::ui::elements::Element;
use mvengine::ui::elements::UiElementStub;
use mvengine::window::Window;
use mvengine_proc::style_expr;
use mvengine_proc::ui;
use mvutils::bytebuffer::ByteBufferExtras;
use mvutils::hashers::U64IdentityHasher;
use mvutils::once::CreateOnce;
use mvutils::save::Savable;
use mvutils::thread::ThreadSafe;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use mvengine::ui::elements::events::UiClickAction;
use crate::ui::display::chat::Chat;

pub struct Game {
    pub world: ClientWorld,
    pub player: ClientPlayer,
    pub other_players: HashMap<ClientId, ClientPlayer, U64IdentityHasher>,
    pub conf_dir: PathBuf,
    pub client_resources: LocalModManager,
    pub available_tiles: Vec<TileKind>,
    pub(crate) tile_size: i32,
    pub selection: Option<TileSelection>,
    prepare_selection: bool,

    click_area: CreateOnce<ThreadSafe<Element>>,
    pub chat: CreateOnce<Chat>
}

impl Game {
    pub fn new() -> Self {
        let appdata = env::var("APPDATA").expect("Failed to get APPDATA environment variable");
        let mut full = PathBuf::from(appdata);
        full.push(input::PATH);

        let local_mods = LocalModManager::new();

        Self {
            world: ClientWorld::new(),
            player: ClientPlayer::new(1, 1, ClientDataPacket {
                name: "v22".to_string(),
                render_distance: 1,
            }),
            other_players: HashMap::with_hasher(U64IdentityHasher::default()),
            conf_dir: full,
            client_resources: local_mods,
            available_tiles: vec![],
            tile_size: 50,
            selection: None,
            prepare_selection: false,
            click_area: CreateOnce::new(),
            chat: CreateOnce::new(),
        }
    }

    pub fn create_ui(&mut self, window: &mut Window) {
        let click_area = ui! {
            <Ui context={window.ui().context()}>
                <Div id="click_area" style="padding: none; margin: none; position: absolute; x: 0; y: 0; width: 100%; height: 100%; background.resource: none; border.resource: none;"/>
            </Ui>
        };
        
        self.chat.create(|| Chat::new(window));
        
        window.ui_mut().add_root(click_area.clone());

        self.click_area.create(|| ThreadSafe::new(click_area));
    }
    
    pub fn load_client_res(&mut self) {
        let client_mod_path = Path::join(&self.conf_dir, "resources");
        if let Err(e) = self.client_resources.load_all(&client_mod_path) {
            error!("Error when loading client resources: {e}");
        };
    }
    
    pub fn on_frame(&mut self, window: &mut Window, client: &Option<FactoryIslandClient>) {
        if self.prepare_selection {
            println!("open sel");
            self.selection = Some(TileSelection::new(window, self.available_tiles.iter().cloned()));
            if let Some(sel) = &self.selection {
                sel.open(window, self.click_area.as_ref().clone());
            }
            self.prepare_selection = false;
        }
        if let Some(sel) = &mut self.selection {
            sel.check_events();
        }
    }
    
    pub fn on_server_state(&mut self, packet: ServerStatePacket) {
        self.available_tiles = packet.tiles;
        self.prepare_selection = true;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.player.resize_view(width, height);
    }

    pub fn player_join(&mut self, player: ClientPlayer, id: ClientId) {
        self.other_players.insert(id, player);
    }

    pub fn player_leave(&mut self, id: ClientId) {
        self.other_players.remove(&id);
    }

    pub fn draw_world(&mut self, ctx: &mut impl RenderContext) {
        self.world.draw(ctx, &self.player.camera.view_area, self.tile_size);
    }
    
    pub fn draw_players(&mut self, ctx: &mut impl RenderContext) {
        for player in self.other_players.values() {
            player.draw_from_other_pov(ctx, &self.player.camera.view_area, self.tile_size);
        }

        self.player.draw(ctx, self.tile_size);
    }

    pub fn check_inputs(&mut self, window: &mut Window, client: &mut FactoryIslandClient) {
        let mut has_moved = false;
        let speed = 0.4;
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
        
        if has_moved {
            self.player.broadcast_position(client);
        }

        if let Some(event) = &self.click_area.get().state().events.click_event {
            if event.button == MouseButton::Left && event.base.action == UiClickAction::Click {
                if let Some(sel) = &self.selection {
                    if let Some(tile) = sel.selected_tile() {
                        let pos = TilePos::from_screen((window.input.mouse_x, window.input.mouse_y), &self.player.camera.view_area, self.tile_size);
                        client.send(ServerBoundPacket::TileSet(TileSetFromClientPacket {
                            pos,
                            tile_id: tile.id,
                            tile_state: 0, //Idk use the default??? lets just say zero is ALWAYS a default lmao
                            orientation: Orientation::North,
                        }));
                        println!("set tile");
                    }
                }
            }
        }
    }

    pub fn configuration_directory(&self) -> &PathBuf {
        &self.conf_dir
    }
}