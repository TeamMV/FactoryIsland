use crate::game::camera::Camera;
use crate::game::world::render;
use crate::game::world::tiles::terrain::{TerrainLayer, TerrainMaterial};
use crate::game::world::tiles::{Orientation, Tile};
use mvengine::rendering::control::RenderController;
use mvutils::save::Savable;
use mvutils::Savable;
use std::fmt::{Debug, Formatter};

pub const CHUNK_SIZE: usize = 64;
pub const CHUNK_TILES: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const RENDER_DISTANCE: i32 = 1;
pub const UNLOAD_DISTANCE: i32 = 4;

pub type ChunkGenerator = fn(&mut Chunk, seed: u32);

#[derive(Clone, Savable)]
pub struct Chunk {
    pub seed: u32,
    #[unsaved]
    pub generate_extra: bool,
    pub chunk_world_x: i32,
    pub chunk_world_z: i32,
    pub terrain: TerrainLayer,
    pub tiles: [Tile; CHUNK_TILES]
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
            generate_extra: true,
            chunk_world_x,
            chunk_world_z,
            terrain: TerrainLayer::new(),
            tiles: [0; CHUNK_TILES].map(|_| Tile::Empty),
        }
    }

    pub fn finalize_generation(&mut self) {
        self.terrain.apply_modifications();
        self.tiles.iter_mut().for_each(Tile::post_init);
    }

    pub fn generate_terrain_at(&mut self, x: usize, z: usize, material: TerrainMaterial, orientation: Orientation) {
        let tile_idx = x + z * CHUNK_SIZE;
        self.terrain.terrain[tile_idx] = material;
        self.terrain.orientation[tile_idx] = orientation;
    }

    pub fn set_terrain_at(&mut self, pos: TilePos, material: TerrainMaterial) {
        self.terrain.set_tile_at(pos.in_chunk_x as u8, pos.in_chunk_z as u8, material);
    }

    pub fn set_tile_at(&mut self, mut tile: Tile, pos: TilePos) {
        let tile_idx = pos.in_chunk_x + pos.in_chunk_z * CHUNK_SIZE;
        tile.post_init();
        self.tiles[tile_idx] = tile;
    }

    pub fn get_tile_at(&self, pos: TilePos) -> &Tile {
        let tile_idx = pos.in_chunk_x + pos.in_chunk_z * CHUNK_SIZE;
        &self.tiles[tile_idx]
    }

    pub fn get_tile_at_mut(&mut self, pos: TilePos) -> &mut Tile {
        let tile_idx = pos.in_chunk_x + pos.in_chunk_z * CHUNK_SIZE;
        &mut self.tiles[tile_idx]
    }

    pub fn draw_tiles(&mut self, controller: &mut RenderController, camera: &Camera) {
        unsafe {
            render::draw_tiles(self, controller, self.chunk_world_x, self.chunk_world_z, camera);
        }
    }
}

#[derive(Clone)]
pub struct TilePos {
    pub raw: (i32, i32),
    pub in_chunk_x: usize,
    pub in_chunk_z: usize,
    pub world_chunk_x: i32,
    pub world_chunk_z: i32
}

impl TilePos {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            raw: (x, z),
            in_chunk_x: x as usize % CHUNK_SIZE,
            in_chunk_z: z as usize % CHUNK_SIZE,
            world_chunk_x: (x as f64 / CHUNK_SIZE as f64).floor() as i32,
            world_chunk_z: (z as f64 / CHUNK_SIZE as f64).floor() as i32,
        }
    }
}

impl From<(i32, i32)> for TilePos {
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Debug for TilePos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("raw:{:?}, cx:{}, cz:{}, wx:{}, wz:{}", self.raw, self.in_chunk_x, self.in_chunk_z, self.world_chunk_x, self.world_chunk_z))
    }
}
