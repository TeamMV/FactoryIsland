use crate::world::tiles::impls::lamp::ClientLampTile;
use crate::world::tiles::LoadedClientTile;
use api::registry::{Registerable, Registry};
use api::world::tiles::pos::TilePos;
use bytebuffer::ByteBuffer;
use mvengine::graphics::Drawable;
use mvengine::ui::geometry::SimpleRect;
use mvengine::ui::rendering::WideRenderContext;
use mvutils::lazy;
use crate::world::tiles::impls::conveyor::ClientConveyorTile;

pub mod lamp;
pub mod wood;
pub mod conveyor;

pub trait ClientStateTile {
    fn load_from_server(&mut self, loader: &mut ByteBuffer) -> Result<(), String>;
    fn save_to_server(&self, saver: &mut ByteBuffer);
    fn get_drawable(&self) -> Drawable;
    fn box_clone(&self) -> Box<dyn ClientStateTile>;
}

pub type CustomDraw = fn(&mut dyn WideRenderContext, &SimpleRect, &TilePos, i32, &LoadedClientTile);

pub struct ClientTile {
    pub id: usize,
    pub base: Drawable,
    pub state: Option<Box<dyn ClientStateTile>>,
    pub drawer: Option<CustomDraw>
}

unsafe impl Send for ClientTile {}
unsafe impl Sync for ClientTile {}

impl Clone for ClientTile {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            base: self.base.clone(),
            state: self.state.as_ref().map(|x| x.box_clone()),
            drawer: self.drawer,
        }
    }
}

pub struct ClientTileCreateInfo {
    base: Drawable,
    state: Option<Box<dyn ClientStateTile>>,
    drawer: Option<CustomDraw>
}

impl ClientTileCreateInfo {
    pub fn no_state(base: Drawable) -> Self {
        Self {
            base,
            state: None,
            drawer: None,
        }
    }

    pub fn stateful<S: ClientStateTile + 'static>(base: Drawable, state: S) -> Self {
        Self {
            base,
            state: Some(Box::new(state)),
            drawer: None,
        }
    }

    pub fn no_state_custom_draw(base: Drawable, drawer: CustomDraw) -> Self {
        Self {
            base,
            state: None,
            drawer: Some(drawer),
        }
    }

    pub fn stateful_custom_draw<S: ClientStateTile + 'static>(base: Drawable, state: S, drawer: CustomDraw) -> Self {
        Self {
            base,
            state: Some(Box::new(state)),
            drawer: Some(drawer),
        }
    }
}

impl Registerable for ClientTile {
    type CreateInfo = ClientTileCreateInfo;

    fn with_id(id: usize, info: Self::CreateInfo) -> Self {
        Self {
            id,
            base: info.base,
            state: info.state,
            drawer: info.drawer,
        }
    }
}

lazy! {
    pub static CLIENT_TILE_REG: Registry<ClientTile> = Registry::new();
}

pub fn register_tiles() {
    CLIENT_TILE_REG.register(ClientTileCreateInfo::no_state(wood::BASE.clone()));
    CLIENT_TILE_REG.register(ClientTileCreateInfo::stateful(lamp::BASE.clone(), ClientLampTile::new()));
    CLIENT_TILE_REG.register(ClientTileCreateInfo::stateful(conveyor::BASE.clone(), ClientConveyorTile::new()));
}