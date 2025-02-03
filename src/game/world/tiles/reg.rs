use mvutils::lazy;
use crate::game::world::tiles::{StaticTile, Tile};
use crate::res::R;

lazy! {
    pub static GRASS: Tile = StaticTile::new(R.texture.tile_grass, 0).to_tile();
    pub static SAND: Tile = StaticTile::new(R.texture.tile_sand, 1).to_tile();
    pub static STONE: Tile = StaticTile::new(R.texture.tile_stone, 2).to_tile();
    pub static WATER: Tile = StaticTile::new(R.texture.tile_water, 3).to_tile();
}