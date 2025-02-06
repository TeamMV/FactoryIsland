pub mod reg;

use mvutils::save::{Loader, Savable, Saver};
use mvutils::Savable;
pub use reg::*;

pub const TILE_SIZE: i32 = 30;

#[derive(Clone, Savable)]
pub enum Tile {
    Static(StaticTile)
}

impl Default for Tile {
    fn default() -> Self {
        Self::Static(StaticTile::new(Material::Air))
    }
}

impl Tile {
    pub fn get_material(&self) -> &Material {
        match self {
            Tile::Static(tile) => &tile.material,
        }
    }

    pub fn use_rle(&self) -> bool {
        matches!(self, Tile::Static(_))
    }
}

#[derive(Clone, Copy, Savable, Eq, PartialEq)]
pub enum Material {
    Air,
    Grass,
    Sand,
    Stone,
    Water,
}

#[derive(Clone, Savable)]
pub struct StaticTile {
    pub material: Material,
    #[unsaved]
    pub texture: usize,
    #[unsaved]
    pub orientation: u8,
}

unsafe impl Send for StaticTile {}
unsafe impl Sync for StaticTile {}

impl StaticTile {
    pub fn new(material: Material) -> Self {
        Self {
            material,
            texture: 0,
            orientation: 0,
        }
    }

    pub fn resolve(&mut self, texture: usize, orientation: u8) {
        self.texture = texture;
        self.orientation = orientation;
    }

    pub fn to_tile(self) -> Tile {
        Tile::Static(self)
    }
}