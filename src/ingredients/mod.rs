use crate::drawutils;
use crate::drawutils::Fill;
use crate::res::R;
use api::registry::{Registerable, Registry};
use api::world::chunk::ToClientObject;
use api::world::tiles::Orientation;
use api::world::{PixelUnit, TileUnit};
use mvengine::color::RgbColor;
use mvengine::graphics::Drawable;
use mvengine::ui::geometry::SimpleRect;
use mvengine::ui::rendering::WideRenderContext;
use mvutils::lazy;

pub const IG_SIZE: PixelUnit = (20, 20);

lazy! {
    pub static CLIENT_INGREDIENT_REG: Registry<ClientIngredient> = Registry::new();
}

#[derive(Clone)]
pub struct ClientIngredient {
    pub id: usize,
    pub texture: Drawable,
    pub override_bg: Option<RgbColor>
}

pub struct ClientIngredientCreateInfo {
    pub texture: Drawable,
    pub override_bg: Option<RgbColor>
}

impl ClientIngredientCreateInfo {
    pub fn new(texture: Drawable) -> Self {
        Self { texture, override_bg: None }
    }

    pub fn with_custom_background(texture: Drawable, override_bg: RgbColor) -> Self {
        Self { texture, override_bg: Some(override_bg) }
    }
}

impl Registerable for ClientIngredient {
    type CreateInfo = ClientIngredientCreateInfo;

    fn with_id(id: usize, info: Self::CreateInfo) -> Self {
        Self { id, texture: info.texture, override_bg: info.override_bg }
    }
}

pub fn register_ingredients() {
    CLIENT_INGREDIENT_REG.register(ClientIngredientCreateInfo::new(Drawable::Texture(R.texture.ingredient_stone)));
}

pub struct LoadedClientIngredient {
    id: usize,
    texture: Drawable,
}

impl LoadedClientIngredient {
    pub fn from_server(to_client: ToClientObject) -> Option<Self> {
        let template = CLIENT_INGREDIENT_REG.create_object(to_client.id as usize)?;
        Some(LoadedClientIngredient {
            id: template.id,
            texture: template.texture,
        })
    }

    pub fn draw(
        &self,
        ctx: &mut impl WideRenderContext,
        view_area: &SimpleRect,
        at: TileUnit,
        tile_size: i32,
        y: f32,
    ) {
        drawutils::draw_in_world_size(
            ctx,
            view_area,
            at,
            IG_SIZE,
            Fill::Drawable(self.texture.clone(), Orientation::North),
            tile_size,
            y,
        );
    }
}
