use crate::game::world::generator::biome::Biome;
use crate::game::world::generator::GeneratedColumn;
use crate::game::world::tiles::machines::bore::BoreMachine;
use crate::game::world::tiles::{PassiveTile, Tile};
use noise::{NoiseFn, Perlin};

pub struct TileGenerator {
    scale: f64,
    noise: Perlin
}

impl TileGenerator {
    pub fn new(scale: f64, seed: u32) -> Self {
        Self {
            scale,
            noise: Perlin::new(seed)
        }
    }

    pub fn generate(&self, x: i32, z: i32, column: &GeneratedColumn) -> Option<Tile> {
        let biome = &column.biome;

        let scale = self.scale * 10.0;
        let pt = (x as f64 * scale, z as f64 * scale);
        let noise_value = self.noise.get([pt.0, pt.1]);
        let mapped = (noise_value + 1.0) / 2.0;
        match biome {
            Biome::Ocean | Biome::DeepOcean => None,
            Biome::Grassland | Biome::TallGrass => {
                if mapped == 1.0 {
                    Some(Tile::Mushroom(PassiveTile::new().into()))
                } else {
                    None
                }
            },
            Biome::Desert | Biome::HotDesert => {
                if mapped == 1.0 {
                    Some(Tile::Cactus(PassiveTile::new().into()))
                } else {
                    None
                }
            },
            Biome::Rocks => {
                if mapped == 1.0 {
                    Some(Tile::Bore(BoreMachine::new().into()))
                } else {
                    None
                }
            }
        }
    }
}