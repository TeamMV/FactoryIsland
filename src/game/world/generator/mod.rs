pub mod biome;
pub mod terrain;

use crate::game::world::chunk::{Chunk, CHUNK_SIZE};
use crate::game::world::generator::biome::{Biome, BiomeGeneratorImpl};
use crate::game::world::generator::terrain::TerrainGeneratorImpl;
use crate::game::world::tiles::terrain::TerrainMaterial;
use noise::{NoiseFn, Perlin, Seedable};
use crate::game::world::tiles::Orientation;

pub trait NoiseProvider {
    fn get_noise(&self, point: [f64; 2]) -> f64;
    fn get_seed(&self) -> u32;
}
impl<T: NoiseFn<f64, 2> + Seedable> NoiseProvider for T {
    fn get_noise(&self, point: [f64; 2]) -> f64 {
        self.get(point)
    }

    fn get_seed(&self) -> u32 {
        self.seed()
    }
}

pub struct GeneratorPipeline {
    biomes: Box<dyn BiomeGenerator>,
    terrain: Box<dyn TerrainGenerator>,
}

impl GeneratorPipeline {
    pub fn new(seed: u32) -> Self {
        Self {
            biomes: Box::new(BiomeGeneratorImpl::new(0.01, seed)),
            terrain: Box::new(TerrainGeneratorImpl::new(0.1, seed)),
        }
    }

    pub fn generate(&self, x: i32, z: i32, seed: u32) -> GeneratedColumn {
        let biome = self.biomes.generate(x, z, seed);
        let material = self.terrain.generate(biome.clone(), x, z, seed);
        let orientation = Orientation::from_u8(self.terrain.gen_orientation(x, z, seed));
        GeneratedColumn::new(biome, material, orientation)
    }

    pub fn generate_chunk(&self, chunk: &mut Chunk) {
        for x in 0..CHUNK_SIZE as i32 {
            for z in 0..CHUNK_SIZE as i32 {
                let world_x = chunk.chunk_world_x * CHUNK_SIZE as i32 + x;
                let world_z = chunk.chunk_world_z * CHUNK_SIZE as i32 + z;
                let gen = self.generate(world_x, world_z, chunk.seed);
                if chunk.generate_extra {
                    // do fancy shit
                }
                chunk.generate_terrain_at(x as usize, z as usize, gen.material, gen.orientation);
            }
        }
        chunk.finalize_generation();
    }

    pub fn set_biomes(&mut self, biomes: impl BiomeGenerator + 'static) {
        self.biomes = Box::new(biomes);
    }

    pub fn set_terrain(&mut self, terrain: impl TerrainGenerator + 'static) {
        self.terrain = Box::new(terrain);
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

pub trait BiomeGenerator {
    fn generate(&self, x: i32, z: i32, seed: u32) -> Biome;
}

pub trait TerrainGenerator {
    fn generate(&self, biome: Biome, x: i32, z: i32, seed: u32) -> TerrainMaterial;

    /// Implement this function for the terrain generation to change the orientation of the textures.
    /// This function does not do anything on any other generator.
    fn gen_orientation(&self, x: i32, z: i32, seed: u32) -> u8 {
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
}