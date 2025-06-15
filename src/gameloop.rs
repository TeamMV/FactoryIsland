use crate::camera::Camera;
use crate::game::Game;
use api::server::{ClientBoundPacket, ServerBoundPacket};
use mvengine::net::client::{Client, ClientHandler};
use mvengine::net::DisconnectReason;
use mvengine::rendering::camera::OrthographicCamera;
use mvengine::rendering::control::RenderController;
use mvengine::rendering::shader::default::DefaultOpenGLShader;
use mvengine::rendering::OpenGLRenderer;
use mvengine::window::app::WindowCallbacks;
use mvengine::window::Window;
use mvutils::once::CreateOnce;
use parking_lot::RwLock;
use std::ops::Deref;
use std::sync::Arc;
use log::{error, info};
use mvengine::color::RgbColor;
use mvengine::input::Input;
use mvengine::rendering::post::OpenGLPostProcessRenderer;
use mvengine::ui::rendering::ctx::DrawContext2D;
use mvengine::ui::rendering::UiRenderer;
use mvengine::ui::timing::TIMING_MANAGER;
use mvutils::remake::Remake;
use mvutils::unsafe_utils::Unsafe;
use api::registry;
use api::server::packets::common::ClientDataPacket;
use crate::{debug, input};
use crate::input::{InputManager, ESCAPE};
use crate::player::ClientPlayer;
use crate::rendering::Shaders;
use crate::res::R;
use crate::ui::manager;
use crate::ui::manager::{GameUiManager, UI_ESCAPE_SCREEN};

pub type FactoryIslandClient = Client<ClientBoundPacket, ServerBoundPacket>;

pub struct GameHandler {
    pub this: CreateOnce<Arc<RwLock<Self>>>,
    pub client: Option<FactoryIslandClient>,

    pub renderer: CreateOnce<OpenGLRenderer>,
    pub shader: CreateOnce<DefaultOpenGLShader>,
    pub controller: CreateOnce<RenderController>,
    pub mv_camera: OrthographicCamera,
    pub post_renderer: CreateOnce<OpenGLPostProcessRenderer>,

    pub shaders: CreateOnce<Shaders>,

    pub game: Game,

    pub ui_manager: CreateOnce<GameUiManager>,
    pub draw_ctx: CreateOnce<DrawContext2D>,

    pub cloud_frame: f32
}

impl GameHandler {
    pub fn new() -> Arc<RwLock<Self>> {
        let cam = OrthographicCamera::new(1, 1); //not have divide by 0

        let this = Self {
            this: CreateOnce::new(),
            client: None,
            renderer: CreateOnce::new(),
            shader: CreateOnce::new(),
            controller: CreateOnce::new(),
            mv_camera: cam,
            post_renderer: CreateOnce::new(),
            shaders: CreateOnce::new(),
            game: Game::new(),
            ui_manager: CreateOnce::new(),
            draw_ctx: CreateOnce::new(),
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

            OpenGLRenderer::prepare(window);
            let renderer = OpenGLRenderer::initialize(window);
            self.renderer.create(|| renderer);

            let mut shader = DefaultOpenGLShader::new();
            shader.make().unwrap();
            shader.bind().unwrap();
            self.shader.create(|| shader);

            let controller = RenderController::new(self.shader.get_program_id());
            self.controller.create(|| controller);

            let post_renderer = OpenGLPostProcessRenderer::new(window.info().width as i32, window.info().height as i32);
            self.post_renderer.create(|| post_renderer);
            
            let shaders = Shaders::new();
            self.shaders.create(|| shaders);

            self.mv_camera.update_projection(window.info().width, window.info().height);

            self.game.resize(window.info().width, window.info().height);

            InputManager::init(&self.game, &mut window.input);
            
            let mut manager = GameUiManager::create_all(window);
            manager.goto(manager::UI_MAIN_SCREEN, window);
            self.ui_manager.create(|| manager);
            
            let renderer = UiRenderer::new(window);
            let ctx = DrawContext2D::new(renderer);
            self.draw_ctx.create(|| ctx);
            
            window.disable_depth_test();
        }
    }

    fn update(&mut self, window: &mut Window, delta_u: f64) {

    }

    fn draw(&mut self, window: &mut Window, delta_t: f64) {
        if let Some(client) = &mut self.client {
            self.game.check_inputs(window, client);
        }
        
        if window.input.was_action(ESCAPE) {
            self.ui_manager.goto(UI_ESCAPE_SCREEN, window);
        }
        
        self.game.on_frame(window, &self.client);

        OpenGLRenderer::clear();
        self.shader.use_program();
        self.game.draw_world(&mut *self.controller);
        let target = self.controller.draw_to_target(window, &self.mv_camera, &mut *self.renderer, &mut self.shader);
        self.post_renderer.set_target(target);
        self.shaders.ssao.use_program();
        self.post_renderer.run_shader(&mut self.shaders.ssao);
        self.shaders.clouds.use_program();
        self.shaders.clouds.uniform_2fv("CAM", &self.mv_camera.position);
        self.shaders.clouds.uniform_1f("FRAME", self.cloud_frame);
        self.post_renderer.run_shader(&mut self.shaders.clouds);
        self.post_renderer.draw_to_screen();
        
        self.game.draw_players(&mut *self.controller);
        self.shader.use_program();
        self.controller.draw(window, &self.mv_camera, &mut *self.renderer, &mut self.shader);


        let unsafe_self = unsafe { Unsafe::cast_mut_static(self) };
        self.ui_manager.check_events(window, unsafe_self);
        window.ui_mut().compute_styles_and_draw(&mut self.draw_ctx);
        self.draw_ctx.draw(window);

        self.cloud_frame += 0.003;

        unsafe { TIMING_MANAGER.post_frame(1.0, 0); }
    }

    fn exiting(&mut self, window: &mut Window) {
        if let Some(client) = &mut self.client {
            client.disconnect(DisconnectReason::Disconnected);
        }
        InputManager::close(&self.game, &mut window.input);
    }

    fn resize(&mut self, window: &mut Window, width: u32, height: u32) {
        self.mv_camera.update_projection(window.info().width, window.info().height);
        self.game.resize(width, height);
        *self.renderer = unsafe { OpenGLRenderer::initialize(window) };
        self.draw_ctx.resize(window);
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
                let unsafe_game = unsafe { Unsafe::cast_static(&self.game) };
                self.game.world.sync(packet, unsafe_game);
            }
            ClientBoundPacket::ChunkData(packet) => {
                let unsafe_game = unsafe { Unsafe::cast_static(&self.game) };
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