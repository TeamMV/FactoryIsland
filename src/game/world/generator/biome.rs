use std::fmt::{Debug, Display, Formatter};
use mvengine::color::RgbColor;
use mvutils::Savable;
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

#[derive(Clone, Debug, Savable)]
pub enum Biome {
    Ocean,
    DeepOcean,
    Grassland,
    TallGrass,
    Desert,
    HotDesert,
    Rocks
}

impl Display for Biome {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl Biome {
    pub fn biome_tint(&self) -> RgbColor {
        match self {
            Biome::Ocean => RgbColor::blue().alpha(10),
            Biome::Grassland => RgbColor::green().alpha(10),
            Biome::Desert => RgbColor::yellow().alpha(10),
            Biome::Rocks => RgbColor::new([100, 100, 100, 10]),
            Biome::DeepOcean => RgbColor::blue().alpha(40),
            Biome::TallGrass => RgbColor::green().alpha(30),
            Biome::HotDesert => RgbColor::yellow().alpha(30)
        }
    }
}

const BIOMES: u8 = 7;

impl BiomeGenerator {
    pub(crate) fn generate(&self, x: i32, z: i32) -> Biome {
        let scale = self.scale;
        let pt = (x as f64 * scale, z as f64 * scale);
        let noise_value = self.noise.get([pt.0, pt.1]);

        //replace with more fancy logic
        let mapped = (noise_value + 1.0) / 2.0;
        let mapped = (mapped * BIOMES as f64).round() as u8;
        match mapped {
            0 => Biome::Desert,
            1 => Biome::HotDesert,
            2 => Biome::Grassland,
            3 => Biome::TallGrass,
            4 => Biome::Ocean,
            5 => Biome::DeepOcean,
            _ => Biome::Rocks
        }
    }
}