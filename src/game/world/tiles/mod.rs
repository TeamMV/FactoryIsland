pub mod terrain;
pub mod machines;

use crate::game::world::tiles::machines::bore::BoreMachine;
use mvengine::graphics::tileset::Pump;
use mvengine::math::vec::Vec4;
use mvengine::rendering::texture::Texture;
use mvengine::ui::context::UiResources;
use mvutils::save::{Loader, Savable, Saver};
use mvutils::Savable;
use std::hint::unreachable_unchecked;
use mvengine::rendering::Transform;

pub const TILE_SIZE: i32 = 30;

#[derive(Clone, Copy, Default, Eq, PartialEq, Savable)]
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

    pub fn apply(&self, uv: [(f32, f32); 4]) -> [(f32, f32); 4] {
        match self {
            Orientation::North => uv,
            Orientation::East => [uv[3], uv[0], uv[1], uv[2]],
            Orientation::South => [uv[2], uv[3], uv[0], uv[1]],
            Orientation::West => [uv[1], uv[2], uv[3], uv[0]],
        }
    }
}

#[derive(Clone, Default, Savable)]
pub enum Tile {
    #[default]
    Empty,
    Bore(Box<BoreMachine>)
}

pub trait TileCallbacks {
    fn post_init(&mut self);
    fn get_texture(&mut self) -> Vec<(&Texture, Vec4, Transform)>;
    fn get_orientation(&self) -> Orientation;
    fn is_transparent(&self) -> bool;
}

impl Tile {
    pub fn post_init(&mut self) {
        match self {
            Tile::Empty => {}
            Tile::Bore(bore) => bore.post_init(),
        }
    }

    pub fn get_texture(&mut self) -> Option<Vec<(&Texture, Vec4, Transform)>> {
        match self {
            Tile::Empty => None,
            Tile::Bore(bore) => Some(bore.get_texture()),
        }
    }

    pub fn get_orientation(&self) -> Orientation {
        match self {
            Tile::Empty => Orientation::North,
            Tile::Bore(bore) => bore.get_orientation(),
        }
    }

    pub fn is_transparent(&self) -> bool {
        match self {
            Tile::Empty => true,
            Tile::Bore(bore) => bore.is_transparent(),
        }
    }
}