use crate::game::world::chunk::{Chunk, CHUNK_SIZE};
use crate::game::world::tiles;
use noise::{NoiseFn, Perlin};

const CHUNK_GRID: bool = false;

pub fn default_generator(chunk: &mut Chunk, seed: u32) {
    let perlin = Perlin::new(seed);

    let noise_scale = 0.05;

    for x in 0..CHUNK_SIZE as i32 {
        for z in 0..CHUNK_SIZE as i32 {
            if CHUNK_GRID {
                if x == 0 && chunk.chunk_world_x == 0 {
                    continue;
                }
                if z == 0 && chunk.chunk_world_z == 0 {
                    continue;
                }

                if x == 0 || z == 0 {
                    chunk.set_tile_at(tiles::WATER.clone(), (x, 0, z).into());
                    continue;
                }
            }

            let x = chunk.chunk_world_x * CHUNK_SIZE as i32 + x;
            let z = chunk.chunk_world_z * CHUNK_SIZE as i32 + z;
            let noise_value = perlin.get([(x as f64) * noise_scale, (z as f64) * noise_scale]);
            let noise_value = (noise_value + 1f64) * 0.5f64;
            chunk.set_tile_at(tiles::WATER.clone(), (x, 0, z).into());
            if noise_value > 0.1 {
                chunk.set_tile_at(tiles::STONE.clone(), (x, 1, z).into());
            }
            if noise_value > 0.3 {
                chunk.set_tile_at(tiles::SAND.clone(), (x, 2, z).into());
            }
            if noise_value > 0.6 {
                chunk.set_tile_at(tiles::GRASS.clone(), (x, 3, z).into());
            }
        }
    }
}