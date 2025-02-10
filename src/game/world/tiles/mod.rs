pub mod terrain;

use std::hint::unreachable_unchecked;
use mvutils::save::{Loader, Savable, Saver};
use mvutils::Savable;

pub const TILE_SIZE: i32 = 30;

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum Orientation {
    #[default]
    North,
    South,
    East,
    West
}

impl Orientation {
    pub fn from_u8(value: u8) -> Self {
        match value & 3 {
            0 => Orientation::North,
            1 => Orientation::South,
            2 => Orientation::East,
            3 => Orientation::West,
            // Guaranteed to be unreachable due to bitwise and
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

#[derive(Clone, Default, Savable)]
pub enum Tile {
    #[default]
    Empty,
    // figure out
}

impl Tile {

}

// pub trait ModdedTile {
//     fn get_children(&self) -> Vec<(i32, i32)>;
//     fn get_texture(&self) -> usize;
//     fn get_uv(&self) -> Vec4;
//
//     fn tick(&mut self);
// }
// pub trait ModdedMultiTile {
//     fn get_parent(&self) -> (i32, i32);
//     fn get_uv(&self) -> Vec4;
// }