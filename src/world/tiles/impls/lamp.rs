use std::ops::Deref;
use bytebuffer::ByteBuffer;
use mvengine::graphics::Drawable;
use mvutils::{lazy, Savable};
use mvutils::save::Savable;
use mvutils::utils::TetrahedronOp;
use crate::res::R;
use crate::world::tiles::impls::ClientStateTile;

lazy! {
    pub static BASE: Drawable = D_OFF.clone();
    pub static D_ON: Drawable = Drawable::TileSet(R.tileset.lamp, R.tile.lamp.on);
    pub static D_OFF: Drawable = Drawable::TileSet(R.tileset.lamp, R.tile.lamp.off);
}

#[derive(Savable, Clone)]
pub struct ClientLampTile {
    on: bool
}

impl ClientLampTile {
    pub fn new() -> Self {
        Self {
            on: false,
        }
    }
}

impl ClientStateTile for ClientLampTile {
    fn load_from_server(&mut self, loader: &mut ByteBuffer) -> Result<(), String> {
        *self = Self::load(loader)?;
        Ok(())
    }

    fn save_to_server(&self, saver: &mut ByteBuffer) {
        self.save(saver);
    }

    fn get_drawable(&self) -> Drawable {
        self.on.yn(&*D_ON, &*D_OFF).clone()
    }

    fn box_clone(&self) -> Box<dyn ClientStateTile> {
        Box::new(self.clone())
    }
}