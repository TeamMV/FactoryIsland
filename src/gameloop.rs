use crate::camera::Camera;
use crate::game::{Game, INTERNAL_IP};
use crate::input::{InputManager, ESCAPE};
use crate::player::ClientPlayer;
use crate::rendering::WorldShaders;
use crate::res::R;
use crate::ui::display::inventory::InventoryDisplay;
use crate::ui::manager;
use crate::ui::manager::{GameUiManager, UI_ESCAPE_SCREEN};
use crate::{gamesettings, ingredients, input, world};
use api::registry;
use api::server::packets::common::{ClientDataPacket, ServerStatePacket};
use api::server::packets::inventory::InventoryDataPacket;
use api::server::{ClientBoundPacket, ServerBoundPacket, ServerSync};
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
use mvengine::ui::context::UiResources;
use mvengine::ui::geometry::shape::shapes;
use mvengine::ui::rendering::UiRenderer;
use mvengine::ui::styles::InheritSupplier;
use mvengine::window::app::WindowCallbacks;
use mvengine::window::Window;
use mvutils::once::CreateOnce;
use mvutils::remake::Remake;
use mvutils::unsafe_utils::Unsafe;
use parking_lot::RwLock;
use std::ops::Deref;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Weak};
use std::time::Instant;

pub type FactoryIslandClient = Client<ClientBoundPacket, ServerBoundPacket>;

pub struct GameHandler {
    pub this: CreateOnce<Weak<RwLock<Self>>>,
    pub client: Option<FactoryIslandClient>,

    pub ui_pipeline: CreateOnce<RenderingPipeline<OpenGLRenderer>>,

    pub game: Game,

    pub ui_manager: CreateOnce<GameUiManager>,

    server_packet: Option<ServerStatePacket>,

    window_packet_queue: Vec<ClientBoundPacket>,

    pub cloud_frame: f32,
    pub server_sync: Option<ServerSync>,
}

impl GameHandler {
    pub fn new(is_internal: bool, sync: Option<ServerSync>) -> Arc<RwLock<Self>> {
        let this = Self {
            this: CreateOnce::new(),
            client: None,
            ui_pipeline: CreateOnce::new(),
            game: Game::new(is_internal),
            ui_manager: CreateOnce::new(),
            server_packet: None,
            window_packet_queue: vec![],
            cloud_frame: 0.0,
            server_sync: sync,
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
            world::multitiles::register_all();
            ingredients::register_ingredients();
            window.ui_mut().init(R.deref().deref());

            let mut ui_pipeline = RenderingPipeline::new_default_opengl(window).unwrap();
            self.ui_pipeline.create(|| ui_pipeline);

            InputManager::init(&self.game, &mut window.input);

            let mut manager = GameUiManager::create_all(window, &self.game);
            manager.goto(manager::UI_MAIN_SCREEN, window);
            self.ui_manager.create(|| manager);

            self.game.initialize();

            if self.game.is_internal {
                debug!("Connecting to internal server...");
                let this = self.this.upgrade().expect("weak to self");
                if let Some(c) = FactoryIslandClient::connect(INTERNAL_IP, this) {
                    self.client = Some(c);
                    self.ui_manager.close_all(window);
                }
            }
        }
    }

    fn update(&mut self, window: &mut Window, delta_u: f64) {
        if let Some(packet) = self.server_packet.take() {
            self.game.on_server_state(window, packet);
        }
        let drained = self.window_packet_queue.drain(..).collect::<Vec<_>>();
        for packet in drained {
            self.handle_packet_with_window(packet, window);
        }
    }

    fn draw(&mut self, window: &mut Window, delta_t: f64) {
        self.game
            .on_frame(window, &mut self.client, &mut self.ui_manager);

        OpenGLRenderer::clear();
        OpenGLRenderer::enable_depth_test();
        OpenGLRenderer::enable_depth_buffer();
        if let Some(view) = &mut self.game.world_view {
            view.draw(
                window,
                None,
                Some(&mut self.ui_pipeline),
                &self.game.settings,
            );
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
        if let Some(sync) = &mut self.server_sync {
            sync.stop();
            sync.lock();
            sync.wait();
        }
    }

    fn resize(&mut self, window: &mut Window, width: u32, height: u32) {
        self.ui_pipeline.resize(window);
        self.game.resize(window);
    }
}

impl ClientHandler<ClientBoundPacket> for GameHandler {
    fn on_connected(&mut self) {}

    fn on_disconnected(&mut self, reason: DisconnectReason) {
        error!("Got disconnected from server, reason: {reason:?}");
    }

    fn on_packet(&mut self, packet: ClientBoundPacket) {
        match packet {
            ClientBoundPacket::ServerState(packet) => {
                self.server_packet = Some(packet);
            }
            packet if packet.needs_window() => self.window_packet_queue.push(packet),
            _ => {
                self.game.check_packet(packet);
            }
        }
    }
}

impl GameHandler {
    pub fn handle_packet_with_window(&mut self, packet: ClientBoundPacket, window: &mut Window) {
        let this = unsafe { Unsafe::cast_lifetime(&self.game) };
        if let Some(view) = &mut self.game.world_view {
            view.check_window_packet(packet, window, this);
        }
    }
}
