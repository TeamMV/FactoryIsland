use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use mvengine::rendering::control::RenderController;
use mvutils::Savable;
use mvutils::save::{Loader, Savable, Saver};
use num_traits::real::Real;
use crate::game::camera::Camera;
use crate::game::world::render;
use crate::game::world::tiles::{Material, StaticTile, Tile, TILE_SIZE};
use crate::res::R;
use crate::WINDOW_SIZE;

pub const CHUNK_SIZE: usize = 64;
pub const CHUNK_TILES: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const RENDER_DISTANCE: i32 = 1;

#[derive(Clone)]
pub(crate) struct Layer {
    pub tiles: [Tile; CHUNK_TILES]
}


impl Savable for Layer {
    fn save(&self, saver: &mut impl Saver) {
        let mut current_counter = 1;
        let mut last_material = self.tiles[0].get_material();

        let save_tiles = |mat: &Material, amount: u32| {
            saver.push_bool(true);
            saver.push_u32(current_counter);
            mat.save(saver);
        };

        for tile in self.tiles.iter().skip(1) {
            if !tile.use_rle() {
                save_tiles(last_material, current_counter);
                current_counter = 0;
                last_material = tile.get_material();
                saver.push_bool(false);
                tile.save(saver);
                continue;
            }
            if tile.get_material() != last_material {
                save_tiles(last_material, current_counter);
                current_counter = 1;
                last_material = tile.get_material();
            } else {
                current_counter += 1;
            }
        }

        if current_counter > 0 {
            save_tiles(last_material, current_counter);
        }
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        let mut tiles = [0; CHUNK_TILES].map(|_| Tile::default());
        let mut index = 0;

        loop {
            if bool::load(loader)? {
                let count = u32::load(loader)?;
                let material = Material::load(loader)?;
                for _ in 0..count {
                    tiles[index] = Tile::Static(StaticTile::new(material));
                    index += 1;
                }
            } else {
                tiles[index] = Tile::load(loader)?;
                index += 1;
            }
            if index >= CHUNK_TILES {
                break
            }
        }

        Ok(Self {
            tiles,
        })
    }
}

impl Layer {
    fn new() -> Self {
        Self {
            tiles: [0; CHUNK_TILES].map(|_| Tile::default()),
        }
    }

    fn set_tile_at(&mut self, tile: Tile, chunk_x: usize, chunk_z: usize) {
        self.tiles[chunk_x + CHUNK_SIZE * chunk_z] = tile;
    }

    fn get_tile_at(&self, chunk_x: usize, chunk_z: usize) -> &Tile {
        &self.tiles[chunk_x + CHUNK_SIZE * chunk_z]
    }

    fn get_tile_at_mut(&mut self, chunk_x: usize, chunk_z: usize) -> &mut Tile {
        &mut self.tiles[chunk_x + CHUNK_SIZE * chunk_z]
    }
}

pub type ChunkGenerator = fn(&mut Chunk, seed: u32);

#[derive(Clone, Savable)]
pub struct Chunk {
    pub seed: u32,
    pub generated: bool,
    pub chunk_world_x: i32,
    pub chunk_world_z: i32,
    pub air_tile: Tile,
    pub layers: Vec<Layer>,
    pub y_levels: [i32; CHUNK_TILES]
}

unsafe impl Send for Chunk {}
unsafe impl Sync for Chunk {}

impl Debug for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Chunk")
            .field("x", &self.chunk_world_x)
            .field("z", &self.chunk_world_z)
            .finish()
    }
}

impl Chunk {
    pub fn new(chunk_world_x: i32, chunk_world_z: i32, seed: u32) -> Self {
        Self {
            seed,
            generated: false,
            chunk_world_x,
            chunk_world_z,
            air_tile: Tile::default(),
            layers: Vec::new(),
            y_levels: [0; CHUNK_TILES],
        }
    }

    pub fn set_tile_at(&mut self, mut tile: Tile, pos: TilePos) {
        let tile_idx = pos.in_chunk_x + pos.in_chunk_z * CHUNK_SIZE;
        if let Tile::default() = tile {
            if pos.raw.1 == self.y_levels[tile_idx] {
                for layer_idx in (0..self.layers.len()).rev() {
                    let layer = &self.layers[layer_idx];
                    let layer_tile = &layer.tiles[tile_idx];
                    if !matches!(layer_tile, Tile::default()) {
                        self.y_levels[tile_idx] = (layer_idx as i32 - 1).max(0);
                        break;
                    }
                }
            }
        } else {
            if pos.raw.1 > self.y_levels[tile_idx] {
                self.y_levels[tile_idx] = pos.raw.1;
            }
        }
        let layer = if let Some(layer) = self.layers.get_mut(pos.layer) {
            layer
        } else {
            let diff = pos.layer - self.layers.len() + 1;
            for _ in 0..diff {
                let layer = Layer::new();
                self.layers.push(layer);
            }
            &mut self.layers[pos.layer]
        };
        if let Tile::Static(ref mut tile) = tile {
            tile.orientation = Self::gen_orientation(pos.raw.0, pos.raw.2, self.seed);
        };
        layer.set_tile_at(tile, pos.in_chunk_x, pos.in_chunk_z)
    }

    fn gen_orientation(x: i32, z: i32, seed: u32) -> u8 {
        let mut hash = seed;
        let x = x as u32;
        let z = z as u32;

        hash = hash.wrapping_add(x);
        hash = hash.wrapping_mul(1610612741);
        hash = hash.wrapping_add(z);
        hash = hash.wrapping_mul(805306457);

        hash ^= hash >> 16;
        hash = hash.wrapping_mul(937412447);
        hash ^= hash >> 13;
        hash = hash.wrapping_mul(1385293057);
        hash ^= hash >> 16;

        (hash & 3) as u8
    }

    pub fn get_tile_at(&self, pos: TilePos) -> &Tile {
        if self.layers.len() >= pos.layer {
            &self.air_tile
        } else {
            self.layers[pos.layer].get_tile_at(pos.in_chunk_x, pos.in_chunk_z)
        }
    }

    pub fn get_tile_at_mut(&mut self, pos: TilePos) -> &mut Tile {
        if self.layers.len() >= pos.layer {
            &mut self.air_tile
        } else {
            self.layers[pos.layer].get_tile_at_mut(pos.in_chunk_x, pos.in_chunk_z)
        }
    }

    pub fn get_y_level(&self, pos: TilePos) -> i32 {
        self.y_levels[pos.in_chunk_x + pos.in_chunk_z * CHUNK_SIZE]
    }

    pub fn request_generate(&mut self, generator: ChunkGenerator) {
        if !self.generated {
            self.generated = true;
            generator(self, self.seed);
        }
    }

    pub fn draw_tiles(&self, controller: &mut RenderController, camera: &Camera) {
        unsafe {
            render::draw_tiles(self, controller, self.chunk_world_x, self.chunk_world_z, camera);
        }
    }
}

#[derive(Clone)]
pub struct TilePos {
    pub raw: (i32, i32, i32),
    pub layer: usize,
    pub in_chunk_x: usize,
    pub in_chunk_z: usize,
    pub world_chunk_x: i32,
    pub world_chunk_z: i32
}

impl TilePos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            raw: (x, y, z),
            layer: y as usize,
            in_chunk_x: x as usize % CHUNK_SIZE,
            in_chunk_z: z as usize % CHUNK_SIZE,
            world_chunk_x: (x as f64 / CHUNK_SIZE as f64).floor() as i32,
            world_chunk_z: (z as f64 / CHUNK_SIZE as f64).floor() as i32,
        }
    }
}

impl From<(i32, i32, i32)> for TilePos {
    fn from(value: (i32, i32, i32)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}




