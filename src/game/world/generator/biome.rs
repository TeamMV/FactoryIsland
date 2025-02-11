use noise::{NoiseFn, Perlin};

pub struct BiomeGenerator {
    scale: f64,
    noise: Perlin
}

impl BiomeGenerator {
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

impl BiomeGenerator {
    pub(crate) fn generate(&self, x: i32, z: i32) -> Biome {
        let scale = self.scale;
        let pt = (x as f64 * scale, z as f64 * scale);
        let noise_value = self.noise.get([pt.0, pt.1]);

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