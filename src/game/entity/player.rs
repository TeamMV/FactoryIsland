use std::ops::Mul;
use mvengine::color::RgbColor;
use mvengine::rendering::control::RenderController;
use mvengine::rendering::{InputVertex, Quad, Transform};
use mvengine::ui::context::UiResources;
use mvengine::ui::rendering::ctx;
use mvengine::window::Window;
use crate::game::camera::Camera;
use crate::game::world::tiles::TILE_SIZE;
use crate::game::world::World;
use crate::res::R;
use crate::WINDOW_SIZE;

const TEXT_HEIGHT: f32 = 20.0;

pub struct WorldPlayer {
    pub name: String,
    pub texture: usize,
    pub position: (f64, f64),
    pub size: (i32, i32),
    pub camera: Camera,
    pub(crate) world: World //prolly change that in the future
}

impl WorldPlayer {
    pub fn new(name: &str, texture: usize, size: (i32, i32), world: World) -> Self {
        unsafe {
            let mut cam = Camera::new();
            cam.x = (WINDOW_SIZE.0 / (TILE_SIZE * 2)) as f64;
            cam.y = (WINDOW_SIZE.1 / (TILE_SIZE * 2)) as f64;
            Self {
                name: name.to_string(),
                texture,
                position: (0.0, 0.0),
                size,
                camera: cam,
                world
            }
        }
    }



    pub fn move_rel(&mut self, dx: f64, dy: f64) {
        self.position.0 += dx;
        self.position.1 += dy;
        self.camera.move_rel(dx, dy);
        self.world.on_cam_move(&self.camera);
    }

    pub fn draw(&self, controller: &mut RenderController, window: &Window) {
        if let Some(tex) = R.resolve_texture(self.texture) {
            let trans = ctx::transform()
                .get();

            let vertex = |xy: (i32, i32), uv: (f32, f32)| -> InputVertex {
                InputVertex {
                    transform: trans.clone().translate_self(xy.0 as f32, xy.1 as f32),
                    pos: (0.0, 0.0, 1.0),
                    color: RgbColor::transparent().as_vec4(),
                    uv,
                    texture: tex.id,
                    has_texture: 1.0,
                }
            };

            let pos = self.screen_pos();

            controller.push_quad(Quad {
                points: [
                    vertex((pos.0, pos.1), (0.0, 0.0)),
                    vertex((pos.0, pos.1 + self.size.1), (0.0, 1.0)),
                    vertex((pos.0 + self.size.0, pos.1 + self.size.1), (1.0, 1.0)),
                    vertex((pos.0 + self.size.0, pos.1), (1.0, 0.0)),
                ],
            });
        }
    }

    pub fn draw_ui(&self, controller: &mut RenderController, window: &Window) {
        if let Some(font) = R.resolve_font(R.font.default) {
            let text_width = font.get_width(&self.name, TEXT_HEIGHT);
            let pos = self.screen_pos();
            let mut trans = Transform::new();
            trans.translation.x = (self.size.0 as f32) / 2.0 - text_width / 2.0 + pos.0 as f32;
            trans.translation.y = self.size.1 as f32 + 10.0 + pos.1 as f32;

            let vertex = |(w, h)| -> InputVertex {
                InputVertex {
                    transform: trans.clone().translate_self(w, h),
                    pos: (0.0, 0.0, 1.0),
                    color: RgbColor::black().as_vec4().mul(0.2),
                    uv: (0.0, 0.0),
                    texture: 0,
                    has_texture: 0.0,
                }
            };

            controller.push_quad(Quad {
                points: [
                    vertex((-5.0, -5.0)),
                    vertex((-5.0, TEXT_HEIGHT + 5.0)),
                    vertex((text_width + 5.0, TEXT_HEIGHT + 5.0)),
                    vertex((text_width + 5.0, -5.0)),
                ],
            });

            font.draw(&self.name, TEXT_HEIGHT, trans, 1.0, &RgbColor::white(), controller);
        }
    }

    fn screen_pos(&self) -> (i32, i32) {
        (
            (self.camera.x * TILE_SIZE as f64) as i32 - (self.position.0 * TILE_SIZE as f64) as i32,
            (self.camera.y * TILE_SIZE as f64) as i32 - (self.position.1 * TILE_SIZE as f64) as i32
        )
    }

    pub fn set_world(&mut self, world: World) {
        self.world = world;
    }
}