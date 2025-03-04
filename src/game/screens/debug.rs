use crate::game::camera::Camera;
use crate::game::world::chunk::{TilePos, CHUNK_SIZE};
use crate::res::R;
use mvengine::color::RgbColor;
use mvengine::rendering::camera::OrthographicCamera;
use mvengine::rendering::control::RenderController;
use mvengine::rendering::light::LightOpenGLRenderer;
use mvengine::rendering::shader::light::LightOpenGLShader;
use mvengine::rendering::Transform;
use mvengine::ui::context::UiResources;
use mvengine::ui::rendering::ctx;
use mvengine::window::Window;
use crate::game;
use crate::game::entity::player::WorldPlayer;
use crate::game::world::{render, World};
use crate::game::world::tiles::TILE_SIZE;

pub struct DebugScreen {
    renderer: LightOpenGLRenderer,
    controller: RenderController,
    shader: LightOpenGLShader,
    mv_camera: OrthographicCamera,
}

impl DebugScreen {
    pub fn new(window: &Window) -> Self {
        unsafe {
            let mut renderer = LightOpenGLRenderer::initialize(window);
            renderer.set_ambient(RgbColor::white().as_vec4());

            let mut shader = LightOpenGLShader::new();
            shader.make().unwrap();
            shader.bind().unwrap();


            Self {
                renderer,
                controller: RenderController::new(shader.get_program_id()),
                shader,
                mv_camera: OrthographicCamera::new(window.info().width, window.info().height),
            }
        }
    }

    pub fn start(&mut self) {
        self.shader.use_program();
    }

    pub fn draw(&mut self, window: &Window, player: &mut WorldPlayer) {
       if let Some(font) = R.resolve_font(R.font.default) {
           let font_size = 50;
           let mut trans = ctx::transform()
               .translate(20, (window.info().height - font_size - 20) as i32)
               .get();

           let tile_pos: TilePos = player.get_world_position().into();

           font.draw(format!("Position: {:.1}, {:.1}", tile_pos.raw.0, tile_pos.raw.1).as_str(), font_size as f32, trans.clone(), 1.0, &RgbColor::white(), &mut self.controller);
           trans.translation.y -= (font_size + 20) as f32;
           font.draw(format!("Chunk:    {}, {} [{}, {}]", tile_pos.world_chunk_x, tile_pos.world_chunk_z, tile_pos.in_chunk_x, tile_pos.in_chunk_z).as_str(), font_size as f32, trans.clone(), 1.0, &RgbColor::white(), &mut self.controller);
           trans.translation.y -= (font_size + 20) as f32;
           let chunk = player.world.force_load(tile_pos.world_chunk_x, tile_pos.world_chunk_z);
           let biome = chunk.get_biome_at(tile_pos.in_chunk_x, tile_pos.in_chunk_z);
           font.draw(format!("Biome:    {}", biome).as_str(), font_size as f32, trans, 1.0, &RgbColor::white(), &mut self.controller);

           game::debug::world_quad(&mut self.controller, tile_pos.raw.0, tile_pos.raw.1, &player.camera, RgbColor::red().alpha(50), Transform::new());
       };
        self.controller.draw(window, &self.mv_camera, &mut self.renderer, &mut self.shader);
    }
}