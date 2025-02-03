use crate::game::camera::Camera;
use crate::game::event::EventDispatcher;
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
use mvengine::window::Window;
use std::ops::Mul;

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
}

impl WorldScreen {
    pub fn new(window: &Window, world: World) -> Self {
        unsafe {
            LightOpenGLRenderer::prepare(window);
            let mut renderer = LightOpenGLRenderer::initialize(window);
            renderer.push_light(Light {
                pos: Vec2::new(300.0, 300.0),
                color: RgbColor::red().as_vec4(),
                intensity: 1.2,
                range: 200.0,
                falloff: 1.8,
            });

            renderer.set_ambient(RgbColor::white().as_vec4());

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
            }
        }
    }

    pub fn draw(&mut self, window: &Window, event_dispatcher: &mut EventDispatcher) {
        let input = &window.input;

        if input.is_action("move_up") {
            self.camera.move_rel(0.0, -0.5);
            self.world.on_cam_move(&self.camera, event_dispatcher);
        }
        if input.is_action("move_down") {
            self.camera.move_rel(0.0, 0.5);
            self.world.on_cam_move(&self.camera, event_dispatcher);
        }
        if input.is_action("move_left") {
            self.camera.move_rel(0.5, 0.0);
            self.world.on_cam_move(&self.camera, event_dispatcher);
        }
        if input.is_action("move_right") {
            self.camera.move_rel(-0.5, 0.0);
            self.world.on_cam_move(&self.camera, event_dispatcher);
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

        self.frame += 0.003;
    }

    pub fn resize(&mut self, window: &Window) {
        unsafe {
            self.renderer = LightOpenGLRenderer::initialize(window);
            self.mv_camera = OrthographicCamera::new(window.info().width, window.info().height);
            self.post_renderer = OpenGLPostProcessRenderer::new(window.info().width as i32, window.info().height as i32);

            self.renderer.push_light(Light {
                pos: Vec2::new(300.0, 300.0),
                color: RgbColor::red().as_vec4(),
                intensity: 1.2,
                range: 200.0,
                falloff: 1.8,
            });

            self.renderer.set_ambient(RgbColor::white().as_vec4());
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

                        let chunk = self.world.get_chunk_mut(&(world_x, 0, world_y).into());
                        let tile_pos: TilePos = (world_x, 0, world_y).into();
                        let y = chunk.get_y_level(tile_pos.clone());
                        let tile_pos = (world_x, y, world_y).into();
                        chunk.set_tile_at(Tile::Air, tile_pos);
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