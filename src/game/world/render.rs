use mvengine::color::RgbColor;
use mvengine::rendering::control::RenderController;
use mvengine::rendering::{InputVertex, Quad, Transform, Vertex};
use mvengine::ui::context::UiResources;
use rand::Rng;
use crate::game::camera::Camera;
use crate::game::world::chunk::{Chunk, CHUNK_SIZE};
use crate::game::world::tiles::{Tile, TILE_SIZE};
use crate::res::R;
use crate::WINDOW_SIZE;

pub unsafe fn draw_tiles(chunk: &Chunk, controller: &mut RenderController, cx: i32, cz: i32, camera: &Camera) {
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let y = chunk.get_y_level((x as i32, 0, z as i32).into());
            //println!("y: {y}");
            if let Some(layer) = chunk.layers.get(y as usize) {
                let tile_x = (cx * CHUNK_SIZE as i32 + x as i32) * TILE_SIZE + (camera.x * TILE_SIZE as f64) as i32;
                let tile_z = (cz * CHUNK_SIZE as i32 + z as i32) * TILE_SIZE + (camera.y * TILE_SIZE as f64) as i32;

                let zoomed_x = (((tile_x as f64 - camera.x * TILE_SIZE as f64) * camera.zoom as f64) + camera.x * TILE_SIZE as f64) as i32;
                let zoomed_z = (((tile_z as f64 - camera.y * TILE_SIZE as f64) * camera.zoom as f64) + camera.y * TILE_SIZE as f64) as i32;

                if zoomed_x >= -TILE_SIZE && zoomed_z >= -TILE_SIZE {
                    if zoomed_x < WINDOW_SIZE.0 && zoomed_z < WINDOW_SIZE.1 {
                        let tile = &layer.tiles[x + z * CHUNK_SIZE];
                        match tile {
                            Tile::Air => {}
                            Tile::Static(s) => {
                                let tex = R.resolve_texture(s.texture as usize).unwrap();

                                let y = y * 100;
                                let w = (TILE_SIZE as f32 * camera.zoom) as i32;
                                let h = (TILE_SIZE as f32 * camera.zoom) as i32;

                                let x1 = zoomed_x as f32;
                                let x2 = zoomed_x as f32 + w as f32;
                                let y1 = zoomed_z as f32;
                                let y2 = zoomed_z as f32 + h as f32;

                                let uv_coords = match s.orientation {
                                    1 => [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)], // 90 degrees
                                    2 => [(1.0, 1.0), (0.0, 1.0), (0.0, 0.0), (1.0, 0.0)], // 180 degrees
                                    3 => [(0.0, 1.0), (0.0, 0.0), (1.0, 0.0), (1.0, 1.0)], // 270 degrees
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
                        }
                    }
                }
            }
        }
    }
}