use mvengine::color::RgbColor;
use mvengine::rendering::control::RenderController;
use mvengine::rendering::{InputVertex, Quad};
use mvengine::ui::context::UiResources;
use mvengine::ui::rendering::ctx;
use mvengine::window::Window;
use crate::game::camera::Camera;
use crate::game::world::tiles::TILE_SIZE;
use crate::res::R;

pub struct Player {
    pub name: String,
    pub texture: usize,
    pub position: (i32, i32)
}

impl Player {
    pub fn new(name: &str, texture: usize) -> Self {
        Self { name: name.to_string(), texture, position: (0, 0) }
    }

    pub fn move_rel(&mut self, dx: i32, dy: i32) {
        self.position.0 += dx;
        self.position.1 += dy;
    }

    pub fn draw(&self, controller: &mut RenderController, window: &Window, camera: &Camera) {
        if let Some(tex) = R.resolve_texture(self.texture) {
            let trans = ctx::transform()
                .get();

            let vertex = |xy: (i32, i32), uv: (f32, f32)| -> InputVertex {
                InputVertex {
                    transform: trans.clone(),
                    pos: (xy.0 as f32, 1.0, xy.1 as f32),
                    color: RgbColor::red().as_vec4(),
                    uv,
                    texture: tex.id,
                    has_texture: 0.0,
                }
            };

            controller.push_quad(Quad {
                points: [
                    vertex(Self::pos(camera, self.position.0, self.position.1), (0.0, 0.0)),
                    vertex(Self::pos(camera, self.position.0, self.position.1 + TILE_SIZE), (0.0, 1.0)),
                    vertex(Self::pos(camera, self.position.0 + TILE_SIZE, self.position.1 + TILE_SIZE), (1.0, 1.0)),
                    vertex(Self::pos(camera, self.position.0 + TILE_SIZE, self.position.1), (1.0, 0.0)),
                ],
            });
        }
    }

    pub fn draw_ui(&self, controller: &mut RenderController, window: &Window, camera: &Camera) {

    }

    fn pos(camera: &Camera, x: i32, y: i32) -> (i32, i32) {
        (
            x + (camera.x * TILE_SIZE as f64) as i32,
            y + (camera.y * TILE_SIZE as f64) as i32
        )
    }
}