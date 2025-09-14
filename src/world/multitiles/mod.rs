use crate::res::R;
use api::multitile::MultiTilePlacement;
use api::player::uuid::UUID;
use api::registry::{Registerable, Registry};
use api::world::tiles::pos::TilePos;
use api::world::TileExtent;
use mvengine::graphics::Drawable;
use mvutils::lazy;

pub struct ClientMultiTilePlacement {
    pub uuid: UUID,
    pub mt_id: usize,
    pub pos: TilePos,
    pub extent: TileExtent,
    pub client_multi_tile: Option<ClientMultiTile>,
}

impl ClientMultiTilePlacement {
    pub fn includes(&self, pos: &TilePos) -> bool {
        pos.raw.0 >= self.pos.raw.0
            && pos.raw.0 < self.pos.raw.0 + self.extent.0
            && pos.raw.1 >= self.pos.raw.1
            && pos.raw.1 < self.pos.raw.1 + self.extent.1
    }
}

impl From<MultiTilePlacement> for ClientMultiTilePlacement {
    fn from(value: MultiTilePlacement) -> Self {
        Self {
            uuid: value.uuid,
            mt_id: value.mt_id,
            pos: value.pos,
            extent: value.extent,
            client_multi_tile: CLIENT_MULTI_REG.create_object(value.mt_id),
        }
    }
}

#[derive(Clone)]
pub struct ClientMultiTile {
    pub id: usize,
    pub base: Drawable,
    pub optional: Option<Drawable>,
}

impl ClientMultiTile {
    pub fn get_relevant_texture(&self, is_wide: bool) -> Drawable {
        if !is_wide {
            if let Some(drawable) = self.optional.clone() {
                return drawable;
            }
        }
        self.base.clone()
    }
}

impl Registerable for ClientMultiTile {
    type CreateInfo = ClientMultiTileCreateInfo;

    fn with_id(id: usize, info: Self::CreateInfo) -> Self {
        Self {
            id,
            base: info.base,
            optional: info.optional,
        }
    }
}

pub struct ClientMultiTileCreateInfo {
    base: Drawable,
    optional: Option<Drawable>,
}

impl ClientMultiTileCreateInfo {
    pub fn single_texture(base: Drawable) -> Self {
        Self {
            base,
            optional: None,
        }
    }

    pub fn multi_texture(wide: Drawable, tall: Drawable) -> Self {
        Self {
            base: wide,
            optional: Some(tall),
        }
    }
}

lazy! {
    pub static CLIENT_MULTI_REG: Registry<ClientMultiTile> = Registry::new();
}

pub fn register_all() {
    CLIENT_MULTI_REG.register(ClientMultiTileCreateInfo::multi_texture(
        Drawable::Texture(R.texture.multitile_test),
        Drawable::Texture(R.texture.multitile_test_up),
    ));
}
