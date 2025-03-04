use crate::enum_tile;
use crate::game::world::tiles::{Orientation, TileCallbacks};
use mvengine::math::vec::Vec4;
use mvengine::rendering::texture::Texture;
use mvengine::rendering::Transform;
use mvutils::once::CreateOnce;
use mvutils::unsafe_utils::UnsafeRef;
use mvutils::Savable;
// ##    01
// *#    *2

#[derive(Clone, Default, Savable)]
struct BigWallCore {
    health: u32,
    orientation: Orientation
}

impl TileCallbacks for BigWallCore {
    fn post_init(&mut self) {

    }

    fn get_texture(&mut self) -> Vec<(&Texture, Vec4, Transform)> {
        todo!()
    }

    fn get_orientation(&self) -> Orientation {
        self.orientation
    }

    fn is_transparent(&self) -> bool {
        false
    }
}

#[derive(Clone, Default, Savable)]
struct BigWallExtra {
    index: u8,
}

impl TileCallbacks for BigWallExtra {
    fn post_init(&mut self) {

    }

    fn get_texture(&mut self) -> Vec<(&Texture, Vec4, Transform)> {
        todo!()
    }

    fn get_orientation(&self) -> Orientation {
        Orientation::North
    }

    fn is_transparent(&self) -> bool {
        false
    }
}

enum_tile! {
    #[derive(Clone, Savable)]
    pub enum BigWall {
        Core(BigWallCore),
        Wall(BigWallExtra)
    }
}

impl Default for BigWall {
    fn default() -> Self {
        Self::Core(BigWallCore::default())
    }
}
