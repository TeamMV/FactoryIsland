pub mod machines;
pub mod buildings;
pub mod nature;

use crate::game::world::chunk::TilePos;
use crate::game::world::tiles::machines::bore::BoreMachine;
use mvengine::graphics::tileset::Pump;
use mvengine::math::vec::Vec4;
use mvengine::rendering::texture::Texture;
use mvengine::rendering::Transform;
use mvengine::ui::context::UiResources;
use mvutils::save::{Loader, Savable, Saver};
use mvutils::Savable;
use std::hint::unreachable_unchecked;
use mvengine::ui::res;
use crate::res::R;

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

pub trait TileCallbacks {
    fn post_init(&mut self);
    fn get_texture(&mut self) -> Vec<(&Texture, Vec4, Transform)>;
    fn get_orientation(&self) -> Orientation;
    fn is_transparent(&self) -> bool;
}

#[macro_export]
macro_rules! enum_tile {
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$var_attr:meta])*
                $var:ident($inner:ty)
            ),*$(,)?
        }
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $(
                $(#[$var_attr])*
                $var($inner)
            ),*
        }

        impl $crate::game::world::tiles::TileCallbacks for $name {
            fn post_init(&mut self) {
                match self {
                    $(
                        $name::$var(tile) => tile.post_init()
                    ),*
                }
            }

            fn get_texture(&mut self) -> Vec<(&Texture, Vec4, Transform)> {
                match self {
                    $(
                        $name::$var(tile) => tile.get_texture()
                    ),*
                }
            }

            fn get_orientation(&self) -> Orientation {
                match self {
                    $(
                        $name::$var(tile) => tile.get_orientation()
                    ),*
                }
            }

            fn is_transparent(&self) -> bool {
                match self {
                    $(
                        $name::$var(tile) => tile.is_transparent()
                    ),*
                }
            }
        }
    };
}

macro_rules! tile_wrapper {
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident {
            $(#[$empty_attr:meta])*
            $empty:ident$(,)?
            $(
                $(#[$var_attr:meta])*
                $var:ident($inner:ty)
            ),*$(,)?
        }
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $(#[$empty_attr])*
            $empty,
            $(
                $(#[$var_attr])*
                $var($inner)
            ),*
        }

        impl $name {
            pub fn post_init(&mut self) {
                match self {
                    $name::$empty => {}
                    $(
                        $name::$var(tile) => tile.post_init()
                    ),*
                }
            }

            pub fn get_texture(&mut self) -> Option<Vec<(&Texture, Vec4, Transform)>> {
                match self {
                    $name::$empty => None,
                    $(
                        $name::$var(tile) => Some(tile.get_texture())
                    ),*
                }
            }

            pub fn get_orientation(&self) -> Orientation {
                match self {
                    $name::$empty => Orientation::North,
                    $(
                        $name::$var(tile) => tile.get_orientation()
                    ),*
                }
            }

            pub fn is_transparent(&self) -> bool {
                match self {
                    $name::$empty => true,
                    $(
                        $name::$var(tile) => tile.is_transparent()
                    ),*
                }
            }
        }
    };
}

tile_wrapper! {
    #[derive(Clone, Default, Savable)]
    pub enum Tile {
        #[default]
        Empty,
        Bore(Box<BoreMachine>),
        Cactus(Box<PassiveTile<5, 500>>), //bro this is so scuffed but it does work ig. cannot use the static R for const generic -> todo: the r! proc macro should create a constant module tree
        Mushroom(Box<PassiveTile<6, 500>>),
    }
}

#[derive(Clone, Savable, Default)]
pub struct PassiveTile<const TEXTURE: usize, const START_HEALTH: u32> {
    health: u32,
    orientation: Orientation
}

impl<const TEXTURE: usize, const START_HEALTH: u32> PassiveTile<TEXTURE, START_HEALTH> {
    pub fn new() -> Self {
        Self {
            health: START_HEALTH,
            orientation: Orientation::default(),
        }
    }
}

impl<const TEXTURE: usize, const START_HEALTH: u32> TileCallbacks for PassiveTile<TEXTURE, START_HEALTH> {
    fn post_init(&mut self) {

    }

    fn get_texture(&mut self) -> Vec<(&Texture, Vec4, Transform)> {
        let tex = R.resolve_texture(TEXTURE + res::CR).unwrap_or(R.resolve_texture(R.mv.texture.missing).unwrap());
        vec![(tex, Vec4::default_uv(), Transform::new())]
    }

    fn get_orientation(&self) -> Orientation {
        self.orientation
    }

    fn is_transparent(&self) -> bool {
        true
    }
}
