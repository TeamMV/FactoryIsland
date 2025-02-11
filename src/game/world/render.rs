use crate::game::camera::Camera;
use crate::game::world::chunk::{Chunk, CHUNK_SIZE};
use crate::game::world::tiles::{Orientation, TILE_SIZE};
use crate::res::R;
use crate::WINDOW_SIZE;
use mvengine::color::RgbColor;
use mvengine::rendering::control::RenderController;
use mvengine::rendering::texture::Texture;
use mvengine::rendering::{InputVertex, Quad, Transform};
use mvengine::ui::context::UiResources;

pub unsafe fn draw_tiles(chunk: &Chunk, controller: &mut RenderController, cx: i32, cz: i32, camera: &Camera) {
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            {
                let tile_x = (cx * CHUNK_SIZE as i32 + x as i32) * TILE_SIZE + (camera.x * TILE_SIZE as f64) as i32;
                let tile_z = (cz * CHUNK_SIZE as i32 + z as i32) * TILE_SIZE + (camera.y * TILE_SIZE as f64) as i32;

                let zoomed_x = (((tile_x as f64 - camera.x * TILE_SIZE as f64) * camera.zoom as f64) + camera.x * TILE_SIZE as f64) as i32;
                let zoomed_z = (((tile_z as f64 - camera.y * TILE_SIZE as f64) * camera.zoom as f64) + camera.y * TILE_SIZE as f64) as i32;

                if zoomed_x >= -TILE_SIZE && zoomed_z >= -TILE_SIZE {
                    if zoomed_x < WINDOW_SIZE.0 && zoomed_z < WINDOW_SIZE.1 {
                        let terrain = chunk.terrain.terrain[x + z * CHUNK_SIZE];
                        let tile = &chunk.tiles[x + z * CHUNK_SIZE];
                        if tile.is_transparent() {
                            if let Some(texture) = terrain.get_texture() {
                                let tex = R.resolve_texture(texture).unwrap();
                                let orientation = chunk.terrain.orientation[x + z * CHUNK_SIZE];
                                let y = terrain.get_y() * 100;

                                draw_texture(controller, camera, zoomed_x, zoomed_z, y, tex, orientation);
                            }
                        }
                        if let Some(texture) = tile.get_texture() {
                            let tex = R.resolve_texture(texture).unwrap();
                            let y = terrain.get_y() * 100 + 100;

                            draw_texture(controller, camera, zoomed_x, zoomed_z, y, tex, Orientation::North)
                        }
                    }
                }
            }
        }
    }
}

unsafe fn draw_texture(controller: &mut RenderController, camera: &Camera, zoomed_x: i32, zoomed_z: i32, y: i32, tex: &Texture, orientation: Orientation) {
    let w = (TILE_SIZE as f32 * camera.zoom) as i32;
    let h = (TILE_SIZE as f32 * camera.zoom) as i32;

    let x1 = zoomed_x as f32;
    let x2 = zoomed_x as f32 + w as f32;
    let y1 = zoomed_z as f32;
    let y2 = zoomed_z as f32 + h as f32;

    let uv_coords = match orientation {
        Orientation::North => [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)], // 90 degrees
        Orientation::South => [(1.0, 1.0), (0.0, 1.0), (0.0, 0.0), (1.0, 0.0)], // 180 degrees
        Orientation::East => [(0.0, 1.0), (0.0, 0.0), (1.0, 0.0), (1.0, 1.0)], // 270 degrees
        _ => [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)], // 0 degrees
    };

    controller.push_quad(Quad {
        points: [
            InputVertex {
                transform: Transform::new(),
                pos: (x1, y1, y as f32),
                color: RgbColor::transparent().as_vec4(),
                uv: uv_coords[0],
                texture: tex.id,
                has_texture: 1.0,
            },
            InputVertex {
                transform: Transform::new(),
                pos: (x1, y2, y as f32),
                color: RgbColor::transparent().as_vec4(),
                uv: uv_coords[1],
                texture: tex.id,
                has_texture: 1.0,
            },
            InputVertex {
                transform: Transform::new(),
                pos: (x2, y2, y as f32),
                color: RgbColor::transparent().as_vec4(),
                uv: uv_coords[2],
                texture: tex.id,
                has_texture: 1.0,
            },
            InputVertex {
                transform: Transform::new(),
                pos: (x2, y1, y as f32),
                color: RgbColor::transparent().as_vec4(),
                uv: uv_coords[3],
                texture: tex.id,
                has_texture: 1.0,
            }
        ],
    });
}