use crate::game::world::chunk::{CHUNK_SIZE, CHUNK_TILES};
use crate::game::world::tiles::Orientation;
use crate::res::R;
use hashbrown::HashMap;
use mvutils::Savable;

use mvutils::save::custom::*;
use mvutils::save::Loader;

#[derive(Clone, Copy, Eq, PartialEq, Default, Savable)]
#[varint]
pub enum TerrainMaterial {
    #[default]
    Air,
    Grass,
    Sand,
    Stone,
    Water,
}

impl TerrainMaterial {
    pub fn get_texture(&self) -> Option<usize> {
        match self {
            TerrainMaterial::Air => None,
            TerrainMaterial::Grass => Some(R.texture.tile_grass),
            TerrainMaterial::Sand => Some(R.texture.tile_sand),
            TerrainMaterial::Stone => Some(R.texture.tile_stone),
            TerrainMaterial::Water => Some(R.texture.tile_water),
        }
    }

    pub fn get_y(&self) -> i32 {
        match self {
            TerrainMaterial::Air => 0,
            TerrainMaterial::Grass => 3,
            TerrainMaterial::Sand => 2,
            TerrainMaterial::Stone => 4,
            TerrainMaterial::Water => 1,
        }
    }
}

#[derive(Clone, Savable)]
pub(crate) struct TerrainLayer {
    #[custom(save = ignore_save, load = empty_terrain)]
    pub terrain: [TerrainMaterial; CHUNK_TILES],
    #[custom(save = ignore_save, load = empty_orientation)]
    pub orientation: [Orientation; CHUNK_TILES],
    pub mods: HashMap<u16, TerrainMaterial>,
}

fn empty_terrain(_: &mut impl Loader) -> Result<[TerrainMaterial; CHUNK_TILES], String> {
    Ok([TerrainMaterial::Air; CHUNK_TILES])
}

fn empty_orientation(_: &mut impl Loader) -> Result<[Orientation; CHUNK_TILES], String> {
    Ok([Orientation::North; CHUNK_TILES])
}

impl TerrainLayer {
    pub fn new() -> Self {
        Self {
            terrain: [0; CHUNK_TILES].map(|_| TerrainMaterial::Air),
            orientation: [0; CHUNK_TILES].map(|_| Orientation::North),
            mods: HashMap::new(),
        }
    }

    fn map(x: u8, z: u8) -> u16 {
        u16::from_be_bytes([x, z])
    }

    fn unmap(id: u16) -> (u8, u8) {
        let bytes = id.to_be_bytes();
        (bytes[0], bytes[1])
    }

    pub fn is_original(&self, x: u8, z: u8) -> bool {
        !self.mods.contains_key(&Self::map(x, z))
    }

    pub fn get_tile_at(&self, x: u8, z: u8) -> TerrainMaterial {
        self.terrain[x as usize + z as usize * CHUNK_SIZE]
    }

    pub fn set_tile_at(&mut self, x: u8, z: u8, material: TerrainMaterial) {
        self.terrain[x as usize + z as usize * CHUNK_SIZE] = material;
        self.mods.insert(Self::map(x, z), material);
    }

    pub fn apply_modifications(&mut self) {
        for (pos, material) in &self.mods {
            let (x, z) = Self::unmap(*pos);
            self.terrain[x as usize + z as usize * CHUNK_SIZE] = *material;
        }
    }
}
