use std::ops::Mul;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::game::camera::Camera;
use crate::game::screens::debug::DebugScreen;
use crate::game::world::chunk::{TilePos, CHUNK_SIZE};
use crate::game::world::tiles::{Tile, TILE_SIZE};
use crate::game::world::World;
use crate::WINDOW_SIZE;
use mvengine::color::RgbColor;
use mvengine::input::collect::InputProcessor;
use mvengine::input::consts::MouseButton;
use mvengine::input::{Input, MouseAction, RawInputEvent};
use mvengine::math::vec::Vec2;
use mvengine::rendering::camera::OrthographicCamera;
use mvengine::rendering::control::RenderController;
use mvengine::rendering::light::{Light, LightOpenGLRenderer};
use mvengine::rendering::post::{OpenGLPostProcessRenderer, OpenGLPostProcessShader};
use mvengine::rendering::shader::light::LightOpenGLShader;
use mvengine::ui::timing::{AnimationState, DelayTask, DurationTask, TIMING_MANAGER};
use mvengine::window::Window;
use mvutils::unsafe_utils::Unsafe;
use crate::game::screens::entity::EntityScreen;
use crate::game::world::tiles::machines::bore::BoreMachine;
use crate::game::world::tiles::terrain::TerrainMaterial;

pub struct WorldScreen {
    enabled: bool,
    renderer: LightOpenGLRenderer,
    controller: RenderController,
    shader: LightOpenGLShader,
    mv_camera: OrthographicCamera,
    post_renderer: OpenGLPostProcessRenderer,
    ssao: OpenGLPostProcessShader,
    clouds: OpenGLPostProcessShader,

    world: World,
    camera: Camera,

    frame: f32,

    debug: DebugScreen,
    is_debug: bool,
    entities: EntityScreen,
}

impl WorldScreen {
    pub fn new(window: &Window, world: World) -> Self {
        unsafe {
            LightOpenGLRenderer::prepare(window);
            let mut renderer = LightOpenGLRenderer::initialize(window);
            renderer.set_ambient(RgbColor::white().as_vec4().mul(0.7));

            renderer.push_light(Light {
                pos: Vec2::new(300.0, 300.0),
                color: RgbColor::green().as_vec4(),
                intensity: 100.0,
                range: 100.0,
                falloff: 0.5,
            });

            let mut shader = LightOpenGLShader::new();
            shader.make().unwrap();
            shader.bind().unwrap();

            let mut camera = Camera::new();

            camera.pivot_x = WINDOW_SIZE.0 / 2 - CHUNK_SIZE as i32 / 2;
            camera.pivot_y = WINDOW_SIZE.1 / 2 - CHUNK_SIZE as i32 / 2;

            let mut ssao = OpenGLPostProcessShader::new(include_str!("../../shaders/ssao.frag"));
            ssao.make().unwrap();
            ssao.bind().unwrap();

            let mut clouds = OpenGLPostProcessShader::new(include_str!("../../shaders/clouds.frag"));
            clouds.make().unwrap();
            clouds.bind().unwrap();

            Self {
                enabled: true,
                renderer,
                controller: RenderController::new(shader.get_program_id()),
                shader,
                mv_camera: OrthographicCamera::new(window.info().width, window.info().height),
                post_renderer: OpenGLPostProcessRenderer::new(window.info().width as i32, window.info().height as i32),
                ssao,
                clouds,
                world,
                camera,
                frame: 0.0,
                debug: DebugScreen::new(window),
                is_debug: false,
                entities: EntityScreen::new(),
            }
        }
    }

    pub fn draw(&mut self, window: &Window) {
        let input = &window.input;

        if input.is_action("move_up") {
            self.camera.move_rel(0.0, -0.5);
            self.world.on_cam_move(&self.camera);
        }
        if input.is_action("move_down") {
            self.camera.move_rel(0.0, 0.5);
            self.world.on_cam_move(&self.camera);
        }
        if input.is_action("move_left") {
            self.camera.move_rel(0.5, 0.0);
            self.world.on_cam_move(&self.camera);
        }
        if input.is_action("move_right") {
            self.camera.move_rel(-0.5, 0.0);
            self.world.on_cam_move(&self.camera);
        }
        
        if input.was_action("save") {

        }

        for light in self.renderer.lights_mut() {
            light.pos.x = (self.camera.x * TILE_SIZE as f64) as f32;
            light.pos.y = (self.camera.y * TILE_SIZE as f64) as f32;
        }

        self.shader.use_program();
        self.world.draw(&mut self.controller, &self.camera);

        let cam_pos = Vec2::new(self.camera.x as f32 * TILE_SIZE as f32, self.camera.y as f32 * TILE_SIZE as f32);

        let target = self.controller.draw_to_target(window, &self.mv_camera, &mut self.renderer, &mut self.shader);
        self.post_renderer.set_target(target);
        self.ssao.use_program();
        self.post_renderer.run_shader(&mut self.ssao);
        self.clouds.use_program();
        self.clouds.uniform_2fv("CAM", &cam_pos);
        self.clouds.uniform_1f("FRAME", self.frame);
        self.post_renderer.run_shader(&mut self.clouds);
        self.post_renderer.draw_to_screen();

        self.shader.use_program();
        self.entities.draw(&mut self.controller, window, &self.camera);
        self.controller.draw(window, &self.mv_camera, &mut self.renderer, &mut self.shader);


        if window.input.was_action("debug") {
            self.is_debug ^= true;
        }

        self.debug.start();

        if self.is_debug {
            self.debug.draw(window, &self.camera);
        }

        self.frame += 0.003;
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn resize(&mut self, window: &Window) {
        unsafe {
            self.renderer = LightOpenGLRenderer::initialize(window);
            self.mv_camera = OrthographicCamera::new(window.info().width, window.info().height);
            self.post_renderer = OpenGLPostProcessRenderer::new(window.info().width as i32, window.info().height as i32);

            self.renderer.push_light(Light {
                pos: Vec2::new(300.0, 300.0),
                color: RgbColor::green().as_vec4(),
                intensity: 1.2,
                range: 200.0,
                falloff: 1.8,
            });

            self.renderer.set_ambient(RgbColor::white().as_vec4().mul(1.2));

            self.debug = DebugScreen::new(window);
        }
    }
}

impl InputProcessor for WorldScreen {
    fn digest_action(&mut self, action: RawInputEvent, input: &Input) {
        unsafe {
            if let RawInputEvent::Mouse(ma) = action {
                let mx = input.mouse_x;
                let my = WINDOW_SIZE.1 - input.mouse_y;
                if let MouseAction::Press(mb) = ma {
                    if let MouseButton::Left = mb {
                        let (world_x, world_y) = World::screen_to_world_pos(mx, my, &self.camera);

                        let pos = TilePos::new(world_x, world_y);
                        let chunk = self.world.force_load(pos.world_chunk_x, pos.world_chunk_z);
                        let tile_pos: TilePos = (world_x, world_y).into();
                        chunk.set_terrain_at(tile_pos, TerrainMaterial::Water);
                    } else if let MouseButton::Right = mb {
                        let (world_x, world_y) = World::screen_to_world_pos(mx, my, &self.camera);

                        let pos = TilePos::new(world_x, world_y);
                        self.world.set_tile_at(pos, Tile::Bore(BoreMachine::new().into()));
                    }
                }
            };
        }
    }

    fn end_frame(&mut self) {

    }

    fn set_enabled(&mut self, state: bool) {
        self.enabled = state;
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}