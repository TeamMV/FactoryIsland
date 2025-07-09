use crate::camera::Camera;
use crate::game::Game;
use crate::input::{InputManager, ESCAPE};
use crate::player::ClientPlayer;
use crate::rendering::Shaders;
use crate::res::R;
use crate::ui::manager;
use crate::ui::manager::{GameUiManager, UI_ESCAPE_SCREEN};
use crate::{debug, gamesettings, input};
use api::registry;
use api::server::packets::common::ClientDataPacket;
use api::server::{ClientBoundPacket, ServerBoundPacket};
use log::{error, info};
use mvengine::color::RgbColor;
use mvengine::input::Input;
use mvengine::math::vec::Vec2;
use mvengine::net::client::{Client, ClientHandler};
use mvengine::net::DisconnectReason;
use mvengine::rendering::camera::OrthographicCamera;
use mvengine::rendering::control::RenderController;
use mvengine::rendering::pipeline::RenderingPipeline;
use mvengine::rendering::post::OpenGLPostProcessRenderer;
use mvengine::rendering::shader::default::DefaultOpenGLShader;
use mvengine::rendering::OpenGLRenderer;
use mvengine::ui::rendering::UiRenderer;
use mvengine::window::app::WindowCallbacks;
use mvengine::window::Window;
use mvutils::once::CreateOnce;
use mvutils::remake::Remake;
use mvutils::unsafe_utils::Unsafe;
use parking_lot::RwLock;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Instant;
use mvengine::ui::geometry::shape::shapes;

pub type FactoryIslandClient = Client<ClientBoundPacket, ServerBoundPacket>;

pub struct GameHandler {
    pub this: CreateOnce<Arc<RwLock<Self>>>,
    pub client: Option<FactoryIslandClient>,

    pub world_pipeline: CreateOnce<RenderingPipeline<OpenGLRenderer>>,
    pub player_pipeline: CreateOnce<RenderingPipeline<OpenGLRenderer>>,
    pub ui_pipeline: CreateOnce<RenderingPipeline<OpenGLRenderer>>,

    pub game: Game,

    pub ui_manager: CreateOnce<GameUiManager>,

    pub cloud_frame: f32
}

impl GameHandler {
    pub fn new() -> Arc<RwLock<Self>> {
        let this = Self {
            this: CreateOnce::new(),
            client: None,
            world_pipeline: CreateOnce::new(),
            player_pipeline: CreateOnce::new(),
            ui_pipeline: CreateOnce::new(),
            game: Game::new(),
            ui_manager: CreateOnce::new(),
            cloud_frame: 0.0,
        };

        let arc = Arc::new(RwLock::new(this));
        let cloned = arc.clone();
        let mut lock = arc.write();
        lock.this.create(|| cloned);
        drop(lock);

        arc
    }
}

impl WindowCallbacks for GameHandler {
    fn post_init(&mut self, window: &mut Window) {
        unsafe {
            R::initialize();
            window.ui_mut().init(R.deref().deref());

            self.game.create_ui(window);
            self.game.load_client_res();
            
            let shaders = Shaders::new();


            //Unwrap here cuz what are ya gonna do without rendering
            let mut world_pipeline = RenderingPipeline::new_default_opengl(window).unwrap();
            world_pipeline.create_post(window);
            let [ssao, clouds] = [shaders.ssao, shaders.clouds];
            world_pipeline.add_post_step(ssao);
            world_pipeline.add_post_step(clouds);

            self.world_pipeline.create(|| world_pipeline);

            let mut player_pipeline = RenderingPipeline::new_default_opengl(window).unwrap();
            self.player_pipeline.create(|| player_pipeline);

            let mut ui_pipeline = RenderingPipeline::new_default_opengl(window).unwrap();
            self.ui_pipeline.create(|| ui_pipeline);

            self.game.resize(window.info().width, window.info().height);

            InputManager::init(&self.game, &mut window.input);
            
            let mut manager = GameUiManager::create_all(window);
            manager.goto(manager::UI_MAIN_SCREEN, window);
            self.ui_manager.create(|| manager);
            
            gamesettings::load_settings(&self.game, &mut self.ui_manager);
        }
    }

    fn update(&mut self, window: &mut Window, delta_u: f64) {

    }

    fn draw(&mut self, window: &mut Window, delta_t: f64) {
        if let Some(client) = &mut self.client {
            self.game.check_inputs(window, client, delta_t);
        }
        if window.input.was_action(ESCAPE) {
            self.ui_manager.goto(UI_ESCAPE_SCREEN, window);
        }

        self.game.on_frame(window, &self.client);

        OpenGLRenderer::clear();
        self.world_pipeline.begin_frame();

        self.game.draw_world(&mut *self.world_pipeline);

        OpenGLRenderer::enable_depth_buffer();
        //draw raw world
        self.world_pipeline.advance(window, |_| {});
        //draw ssao
        self.world_pipeline.advance(window, |_| {});
        let cam_pos = Vec2::from_i32s(self.game.player.camera.pos);
        //draw clouds
        self.world_pipeline.advance(window, |s| {
            s.uniform_1f("FRAME", self.cloud_frame);
            s.uniform_2fv("CAM", &cam_pos);
        });

        self.world_pipeline.next_pipeline(&mut *self.player_pipeline);

        self.game.draw_players(&mut *self.player_pipeline);
        self.player_pipeline.advance(window, |_| {});

        let unsafe_self = unsafe { Unsafe::cast_lifetime_mut(self) };
        self.ui_manager.check_events(window, unsafe_self);

        self.player_pipeline.next_pipeline(&mut *self.ui_pipeline);

        let a = window.area();
        window.ui_mut().draw(&mut *self.ui_pipeline, &a);
        OpenGLRenderer::enable_depth_test();
        OpenGLRenderer::enable_depth_buffer();
        self.ui_pipeline.advance(window, |_| {});

        self.cloud_frame += 0.003;
    }

    fn post_draw(&mut self, window: &mut Window, delta_t: f64) {
        //println!("FPS: {}, delta: {}", window.fps(), delta_t);
        mvengine::debug::print_summary(1000);
    }

    fn exiting(&mut self, window: &mut Window) {
        if let Some(client) = &mut self.client {
            client.disconnect(DisconnectReason::Disconnected);
        }
        InputManager::close(&self.game, &mut window.input);
        gamesettings::save_settings(&self.game, &self.ui_manager);
    }

    fn resize(&mut self, window: &mut Window, width: u32, height: u32) {
        self.world_pipeline.resize(window);
        self.player_pipeline.resize(window);
        self.ui_pipeline.resize(window);
        self.game.resize(width, height);
    }
}

impl ClientHandler<ClientBoundPacket> for GameHandler {
    fn on_connected(&mut self) {
        if let Some(client) = &mut self.client {
            client.send(ServerBoundPacket::ClientData(self.game.player.data.clone()));
        }
    }

    fn on_disconnected(&mut self, reason: DisconnectReason) {
        error!("Got disconnected from server, reason: {reason:?}");
    }

    fn on_packet(&mut self, packet: ClientBoundPacket) {
        match packet {
            ClientBoundPacket::TileSet(packet) => {
                let unsafe_game = unsafe { Unsafe::cast_lifetime(&self.game) };
                self.game.world.sync(packet, unsafe_game);
            }
            ClientBoundPacket::ChunkData(packet) => {
                let unsafe_game = unsafe { Unsafe::cast_lifetime(&self.game) };
                self.game.world.sync_chunk(packet, unsafe_game);
            }
            ClientBoundPacket::PlayerMove(packet) => {
                self.game.player.move_to(packet.pos, self.game.tile_size);
            }
            ClientBoundPacket::OtherPlayerMove(packet) => {
                if let Some(player) = self.game.other_players.get_mut(&packet.client_id) {
                    player.move_to(packet.pos, self.game.tile_size);
                }
            }
            ClientBoundPacket::OtherPlayerJoin(packet) => {
                let player = ClientPlayer::new(self.game.player.camera.width, self.game.player.camera.height, packet.client_data);
                self.game.player_join(player, packet.client_id);
            }
            ClientBoundPacket::OtherPlayerLeave(packet) => {
                self.game.player_leave(packet.client_id);
            }
            ClientBoundPacket::ServerState(packet) => {
                for mod_id in &packet.mods {
                    if !self.game.client_resources.is_res_loaded(mod_id) {
                        error!("Server requested resources for mod: {mod_id}, but its resources are not loaded!");
                        if let Some(client) = &mut self.client {
                            client.disconnect(DisconnectReason::Disconnected);
                        }
                        return;
                    }
                }
                
                for player_data in &packet.players {
                    let player = ClientPlayer::new(self.game.player.camera.width, self.game.player.camera.height, player_data.data.clone());
                    self.game.other_players.insert(player_data.client_id, player);
                }
                self.game.on_server_state(packet);
            }
            ClientBoundPacket::ChunkUnload(packet) => {
                self.game.world.drop_chunk(packet.pos);
            }
            ClientBoundPacket::OtherPlayerChat(packet) => {
                self.game.chat.push_message(packet);
            }
        }
    }
}