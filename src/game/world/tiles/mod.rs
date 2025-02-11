pub mod terrain;
pub mod machines;

use std::hint::unreachable_unchecked;
use mvutils::save::{Loader, Savable, Saver};
use mvutils::Savable;
use crate::game::world::tiles::machines::bore::BoreMachine;
use crate::res::R;

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
    Bore(BoreMachine)
}

impl Tile {
    pub fn get_texture(&self) -> Option<usize> {
        match self {
            Tile::Empty => None,
            Tile::Bore(_) => Some(R.texture.machine_bore),
        }
    }

    pub fn is_transparent(&self) -> bool {
        true
    }
}