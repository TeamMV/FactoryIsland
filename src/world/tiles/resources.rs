use mvengine::graphics::Drawable;
use crate::res::R;
use api::tileset;
use api::world::tiles::resources::ClientTileRes;
use mvutils::lazy;

lazy! {
    pub static LAMP_RES: ClientTileRes = ClientTileRes::of(0, tileset!(lamp.off)).and(1, tileset!(lamp.on));
}