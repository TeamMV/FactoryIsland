use crate::res::R;
use crate::world::tiles::impls::ClientStateTile;
use bytebuffer::ByteBuffer;
use mvengine::graphics::Drawable;
use mvutils::save::Savable;
use mvutils::{lazy, Savable};
use api::ingredients::IngredientStack;

lazy! {
    pub static BASE: Drawable = Drawable::TileSet(R.tileset.conveyor, R.tile.conveyor.base);
}

#[derive(Savable, Clone)]
pub struct ClientConveyorTile {
    ingredients: [Option<IngredientStack>; 3],
}

impl ClientConveyorTile {
    pub fn new() -> Self {
        Self {
            ingredients: [None, None, None],
        }
    }
}

impl ClientStateTile for ClientConveyorTile {
    fn load_from_server(&mut self, loader: &mut ByteBuffer) -> Result<(), String> {
        *self = Self::load(loader)?;
        Ok(())
    }

    fn get_drawable(&self) -> Drawable {
        Drawable::Animation(R.animation.conveyor)
    }

    fn box_clone(&self) -> Box<dyn ClientStateTile> {
        Box::new(self.clone())
    }
}
