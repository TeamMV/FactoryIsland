use crate::camera::Camera;
use crate::game::Game;
use crate::input::{InputManager, ESCAPE};
use crate::player::ClientPlayer;
use crate::rendering::WorldShaders;
use crate::res::R;
use crate::ui::manager;
use crate::ui::manager::{GameUiManager, UI_ESCAPE_SCREEN};
use crate::{debug, gamesettings, input, world};
use api::registry;
use api::server::packets::common::{ClientDataPacket, ServerStatePacket};
use api::server::{ClientBoundPacket, ServerBoundPacket};
use log::{debug, error, info};
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
use mvengine::rendering::{OpenGLRenderer, CLEAR_FLAG};
use mvengine::ui::rendering::UiRenderer;
use mvengine::window::app::WindowCallbacks;
use mvengine::window::Window;
use mvutils::once::CreateOnce;
use mvutils::remake::Remake;
use mvutils::unsafe_utils::Unsafe;
use parking_lot::RwLock;
use std::ops::Deref;
use std::sync::{Arc, Weak};
use std::sync::atomic::Ordering;
use std::time::Instant;
use mvengine::ui::context::UiResources;
use mvengine::ui::geometry::shape::shapes;

pub type FactoryIslandClient = Client<ClientBoundPacket, ServerBoundPacket>;

pub struct GameHandler {
    pub this: CreateOnce<Weak<RwLock<Self>>>,
    pub client: Option<FactoryIslandClient>,

    pub ui_pipeline: CreateOnce<RenderingPipeline<OpenGLRenderer>>,

    pub game: Game,

    pub ui_manager: CreateOnce<GameUiManager>,

    server_packet: Option<ServerStatePacket>,

    pub cloud_frame: f32
}

impl GameHandler {
    pub fn new() -> Arc<RwLock<Self>> {
        let this = Self {
            this: CreateOnce::new(),
            client: None,
            ui_pipeline: CreateOnce::new(),
            game: Game::new(),
            ui_manager: CreateOnce::new(),
            server_packet: None,
            cloud_frame: 0.0,
        };

        let arc = Arc::new(RwLock::new(this));
        let cloned = Arc::downgrade(&arc);
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
            world::tiles::impls::register_tiles();
            window.ui_mut().init(R.deref().deref());

            self.game.load_client_res();


            let mut ui_pipeline = RenderingPipeline::new_default_opengl(window).unwrap();
            self.ui_pipeline.create(|| ui_pipeline);

            InputManager::init(&self.game, &mut window.input);

            self.game.initialize();
            
            let mut manager = GameUiManager::create_all(window, &self.game);
            manager.goto(manager::UI_MAIN_SCREEN, window);
            self.ui_manager.create(|| manager);
        }
    }

    fn update(&mut self, window: &mut Window, delta_u: f64) {
        if let Some(packet) = self.server_packet.take() {
            self.game.on_server_state(window, packet);
        }
    }

    fn draw(&mut self, window: &mut Window, delta_t: f64) {
        self.game.on_frame(window, &mut self.client, &mut self.ui_manager);

        OpenGLRenderer::clear();
        OpenGLRenderer::enable_depth_test();
        OpenGLRenderer::enable_depth_buffer();
        if let Some(view) = &mut self.game.world_view {
            view.draw(window, None, Some(&mut self.ui_pipeline), &self.game.settings);
        } else {
            self.ui_pipeline.begin_frame();
        }

        let unsafe_self = unsafe { Unsafe::cast_lifetime_mut(self) };
        self.ui_manager.check_events(window, unsafe_self);

        let a = window.area();
        window.ui_mut().draw(&mut *self.ui_pipeline, &a);
        //OpenGLRenderer::disable_depth_test();
        //OpenGLRenderer::enable_depth_buffer();
        self.ui_pipeline.advance(window, |_| {});
        self.ui_pipeline.flush();

        R.tick_all_animations();

        self.cloud_frame += 0.003;
    }

    fn post_draw(&mut self, window: &mut Window, delta_t: f64) {
        //mvengine::debug::print_summary(1000);
    }

    fn exiting(&mut self, window: &mut Window) {
        if let Some(client) = &mut self.client {
            client.disconnect(DisconnectReason::Disconnected);
        }
        InputManager::close(&self.game, &mut window.input);
        self.game.exit();
    }

    fn resize(&mut self, window: &mut Window, width: u32, height: u32) {
        self.ui_pipeline.resize(window);
        self.game.resize(window);
    }
}

impl ClientHandler<ClientBoundPacket> for GameHandler {
    fn on_connected(&mut self) {

    }

    fn on_disconnected(&mut self, reason: DisconnectReason) {
        error!("Got disconnected from server, reason: {reason:?}");
    }

    fn on_packet(&mut self, packet: ClientBoundPacket) {
        match packet {
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

                self.server_packet = Some(packet);
            }
            _ => {
                self.game.check_packet(packet);
            }
        }
    }
}