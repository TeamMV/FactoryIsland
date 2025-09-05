use bytebuffer::ByteBuffer;
use mvengine::graphics::Drawable;
use mvutils::{lazy, Savable};
use mvutils::save::Savable;
use crate::res::R;
use crate::world::tiles::impls::ClientStateTile;

lazy! {
    pub static BASE: Drawable = Drawable::TileSet(R.tileset.conveyor, R.tile.conveyor.base);
}

#[derive(Savable, Clone)]
pub struct ClientConveyorTile {
    ingredients: Vec<usize>
}

impl ClientConveyorTile {
    pub fn new() -> Self {
        Self {
            ingredients: vec![],
        }
    }
}

impl ClientStateTile for ClientConveyorTile {
    fn load_from_server(&mut self, loader: &mut ByteBuffer) -> Result<(), String> {
        self.ingredients = Vec::load(loader)?;
        Ok(())
    }

    fn save_to_server(&self, saver: &mut ByteBuffer) {
        self.ingredients.save(saver);
    }

    fn get_drawable(&self) -> Drawable {
        Drawable::Animation(R.animation.conveyor)
    }

    fn box_clone(&self) -> Box<dyn ClientStateTile> {
        Box::new(self.clone())
    }
}