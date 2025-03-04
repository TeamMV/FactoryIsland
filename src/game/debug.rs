use mvengine::color::RgbColor;
use mvengine::rendering::control::RenderController;
use mvengine::rendering::{InputVertex, Quad, Transform};
use crate::game::camera::Camera;
use crate::game::world::tiles::TILE_SIZE;

pub fn screen_quad(controller: &mut RenderController, x: i32, y: i32, color: RgbColor, transform: Transform) {
    let x1 = x as f32;
    let x2 = x as f32 + TILE_SIZE as f32;
    let y1 = y as f32;
    let y2 = y as f32 + TILE_SIZE as f32;

    controller.push_quad(Quad {
        points: [
            InputVertex {
                transform: transform.clone().translate_self(x1, y1),
                pos: (0.0, 0.0, y as f32),
                color: color.as_vec4(),
                uv: (0.0, 0.0),
                texture: 0,
                has_texture: 0.0,
            },
            InputVertex {
                transform: transform.clone().translate_self(x1, y2),
                pos: (0.0, 0.0, y as f32),
                color: color.as_vec4(),
                uv: (0.0, 0.0),
                texture: 0,
                has_texture: 0.0,
            },
            InputVertex {
                transform: transform.clone().translate_self(x2, y2),
                pos: (0.0, 0.0, y as f32),
                color: color.as_vec4(),
                uv: (0.0, 0.0),
                texture: 0,
                has_texture: 0.0,
            },
            InputVertex {
                transform: transform.translate_self(x2, y1),
                pos: (0.0, 0.0, y as f32),
                color: color.as_vec4(),
                uv: (0.0, 0.0),
                texture: 0,
                has_texture: 0.0,
            }
        ],
    });
}

pub fn world_quad(controller: &mut RenderController, world_x: i32, world_z: i32, camera: &Camera, color: RgbColor, transform: Transform) {
    let transformed_x = ((world_x as f64 + camera.x) * TILE_SIZE as f64) as i32;
    let transformed_z = ((world_z as f64 + camera.y) * TILE_SIZE as f64) as i32;
    screen_quad(controller, transformed_x, transformed_z, color, transform);
}