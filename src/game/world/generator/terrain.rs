use crate::game::world::generator::biome::Biome;
use crate::game::world::terrain::TerrainMaterial;
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
        let mapped = (noise_value + 1.0) / 2.0;

        match biome {
            Biome::Rocks => {
                if mapped > 0.8 {
                    TerrainMaterial::Water
                } else {
                    TerrainMaterial::Stone
                }
            },
            Biome::Ocean | Biome::DeepOcean => {
                TerrainMaterial::Water
            }
            Biome::Grassland | Biome::TallGrass => {
                TerrainMaterial::Grass
            }
            Biome::Desert | Biome::HotDesert => {
                TerrainMaterial::Sand
            }
        }
    }
}