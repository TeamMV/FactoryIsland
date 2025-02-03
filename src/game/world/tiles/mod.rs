pub mod reg;

use mvutils::save::{Loader, Savable, Saver};
use mvutils::Savable;
pub use reg::*;

pub const TILE_SIZE: i32 = 30;

#[derive(Clone, Savable)]
pub enum Tile {
    Air,
    Static(StaticTile)
}

#[derive(Clone, Savable)]
pub struct StaticTile {
    pub id: u64,
    pub texture: u64,
    pub orientation: u8,
}

unsafe impl Send for StaticTile {}
unsafe impl Sync for StaticTile {}

impl StaticTile {
    pub fn new(texture: usize, id: u64) -> Self {
        Self {
            id,
            texture: texture as u64,
            orientation: 0,
        }
    }

    pub fn to_tile(self) -> Tile {
        Tile::Static(self)
    }
}