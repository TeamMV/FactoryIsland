use crate::game::world::generator::biome::Biome;
use crate::game::world::tiles::terrain::TerrainMaterial;
use noise::{NoiseFn, Perlin};

pub struct TerrainGenerator {
    scale: f64,
    noise: Perlin
}

impl TerrainGenerator {
    pub fn new(scale: f64, seed: u32) -> Self {
        Self {
            scale,
            noise: Perlin::new(seed)
        }
    }
}

impl TerrainGenerator {
    pub(crate) fn generate(&self, biome: Biome, x: i32, z: i32) -> TerrainMaterial {
        let scale = self.scale;
        let pt = (x as f64 * scale, z as f64 * scale);
        let noise_value = self.noise.get([pt.0, pt.1]);

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