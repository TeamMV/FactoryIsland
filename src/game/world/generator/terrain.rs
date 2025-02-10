use crate::game::world::generator::biome::Biome;
use crate::game::world::generator::{BiomeGeneratorImpl, NoiseProvider, TerrainGenerator};
use crate::game::world::tiles::terrain::TerrainMaterial;
use noise::{NoiseFn, Perlin, Seedable};

pub struct TerrainGeneratorImpl {
    scale: f64,
    noise: Perlin
}

impl TerrainGeneratorImpl {
    pub fn new(scale: f64, seed: u32) -> Self {
        Self {
            scale,
            noise: Perlin::new(seed)
        }
    }
}

impl TerrainGenerator for TerrainGeneratorImpl {
    fn generate(&self, biome: Biome, x: i32, z: i32, seed: u32) -> TerrainMaterial {
        let scale = self.scale;
        let pt = (x as f64 * scale, z as f64 * scale);
        let noise_value = self.noise.get_noise([pt.0, pt.1]);

        //replace with more fancy logic
        let mapped = (noise_value + 1.0) / 2.0;
        let mapped = (mapped * 4.0) as u8 & 3;
        match mapped {
            0 => TerrainMaterial::Stone,
            1 => TerrainMaterial::Grass,
            2 => TerrainMaterial::Sand,
            _ => TerrainMaterial::Water,
        }
    }
}