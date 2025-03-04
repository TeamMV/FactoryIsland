use mvengine::graphics::tileset::{ClockingFramePump, LoopingFramePump, Pump};
use mvengine::math::vec::Vec4;
use mvengine::rendering::texture::Texture;
use mvengine::rendering::Transform;
use mvengine::ui::context::UiResources;
use mvengine::ui::res::OrMissingTexture;
use mvutils::{lazy, Savable};
use mvutils::utils::TetrahedronOp;
use crate::game::world::chunk::TilePos;
use crate::game::world::tiles::{Orientation, TileCallbacks};
use crate::game::world::World;
use crate::res::R;

#[derive(Clone, Savable)]
pub struct BoreMachine {
    orientation: Orientation,
    enabled: bool,
    speed: f64,
}

impl BoreMachine {
    pub fn new() -> Self {
        Self {
            orientation: Orientation::North,
            enabled: rand::random::<bool>(),
            speed: rand::random::<bool>().yn(5.0, rand::random::<bool>().yn(10.0, 20.0)),
        }
    }
}

impl TileCallbacks for BoreMachine {
    fn post_init(&mut self) {
        let set = R.resolve_tileset(R.tileset.bore).unwrap();
    }

    fn get_texture(&mut self) -> Vec<(&Texture, Vec4, Transform)> {
        let a = if self.enabled {
            if self.speed > 15.0 {
                R.resolve_animation(R.animation.bore_overloaded).unwrap().get_current()
            } else if self.speed > 7.5 {
                R.resolve_animation(R.animation.bore_fast).unwrap().get_current()
            } else {
                R.resolve_animation(R.animation.bore).unwrap().get_current()
            }
        } else {
            R.resolve_tile(R.tileset.bore, R.tile.bore.disabled).or_missing_texture()
        };
        vec![(a.0, a.1, Transform::new())]
    }

    fn get_orientation(&self) -> Orientation {
        self.orientation
    }

    fn is_transparent(&self) -> bool {
        true
    }
}