use noise::{NoiseFn, Perlin, Seedable};
use crate::game::world::generator::{BiomeGenerator, NoiseProvider};

pub struct BiomeGeneratorImpl {
    scale: f64,
    noise: Perlin
}

impl BiomeGeneratorImpl {
    pub fn new(scale: f64, seed: u32) -> Self {
        Self {
            scale,
            noise: Perlin::new(seed),
        }
    }
}

#[derive(Clone)]
pub enum Biome {
    Ocean,
    Grassland,
    Desert,
    Rocks
}

impl BiomeGenerator for BiomeGeneratorImpl {
    fn generate(&self, x: i32, z: i32, seed: u32) -> Biome {
        let scale = self.scale;
        let pt = (x as f64 * scale, z as f64 * scale);
        let noise_value = self.noise.get_noise([pt.0, pt.1]);

        //replace with more fancy logic
        let mapped = (noise_value + 1.0) / 2.0;
        let mapped = (mapped * 4.0) as u8 & 3;
        match mapped {
            0 => Biome::Grassland,
            1 => Biome::Desert,
            2 => Biome::Rocks,
            _ => Biome::Ocean
        }
    }
}