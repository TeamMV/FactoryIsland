pub mod biome;
pub mod terrain;

use std::hash::{DefaultHasher, Hash, Hasher};
use crate::game::world::chunk::{Chunk, CHUNK_SIZE};
use crate::game::world::generator::biome::{Biome, BiomeGenerator};
use crate::game::world::generator::terrain::TerrainGenerator;
use crate::game::world::tiles::terrain::TerrainMaterial;
use crate::game::world::tiles::Orientation;
use noise::{NoiseFn, Seedable};

const BIOME_SCALE: f64 = 0.01;
const TERRAIN_SCALE: f64 = 0.1;

pub struct GeneratorPipeline {
    seed: u32,
    biomes: BiomeGenerator,
    terrain: TerrainGenerator,
}

impl GeneratorPipeline {
    pub fn new(seed: u32, settings: WorldSettings) -> Self {
        Self {
            seed,
            biomes: BiomeGenerator::new(settings.biome_scale, seed),
            terrain: TerrainGenerator::new(settings.terrain_scale, seed),
        }
    }

    pub fn generate(&mut self, x: i32, z: i32) -> GeneratedColumn {
        let biome = self.biomes.generate(x, z);
        let material = self.terrain.generate(biome.clone(), x, z);
        let orientation = Orientation::from_u8(self.gen_orientation(x, z));
        GeneratedColumn::new(biome, material, orientation)
    }

    pub fn generate_chunk(&mut self, chunk: &mut Chunk) {
        for x in 0..CHUNK_SIZE as i32 {
            for z in 0..CHUNK_SIZE as i32 {
                let world_x = chunk.chunk_world_x * CHUNK_SIZE as i32 + x;
                let world_z = chunk.chunk_world_z * CHUNK_SIZE as i32 + z;
                let gen = self.generate(world_x, world_z);
                if chunk.generate_extra {
                    // do fancy shit
                }
                chunk.generate_terrain_at(x as usize, z as usize, gen.material, gen.orientation);
            }
        }
        chunk.finalize_generation();
    }

    fn gen_orientation(&self, x: i32, z: i32) -> u8 {
        let mut hash = self.seed;
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
}

pub struct GeneratedColumn {
    pub biome: Biome,
    pub material: TerrainMaterial,
    pub orientation: Orientation,
}

impl GeneratedColumn {
    pub fn new(biome: Biome, material: TerrainMaterial, orientation: Orientation) -> Self {
        Self { biome, material, orientation }
    }
}

pub struct WorldSettings {
    pub seed: u32,
    pub terrain_scale: f64,
    pub biome_scale: f64
}

impl WorldSettings {
    pub fn from_seed(seed: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let seed = hasher.finish() as u32;
        let mut this = WorldSettings::default();
        this.seed = seed;
        this
    }
}

impl Default for WorldSettings {
    fn default() -> Self {
        WorldSettings {
            seed: 22,
            terrain_scale: TERRAIN_SCALE,
            biome_scale: BIOME_SCALE,
        }
    }
}