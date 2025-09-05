use std::ops::Deref;
use mvengine::graphics::Drawable;
use mvengine::rendering::RenderContext;
use mvengine::ui::geometry::SimpleRect;
use mvengine::ui::rendering::adaptive::AdaptiveFill;
use mvengine::ui::rendering::WideRenderContext;
use mvengine_proc::resolve_resource;
use mvutils::lazy;
use api::registry::{ObjectSource, Registerable, Registry};
use api::world::chunk::ToClientObject;
use api::world::{PixelUnit, TileUnit};
use api::world::tiles::Orientation;
use crate::drawutils;
use crate::drawutils::Fill;
use crate::world::tiles::ClientDrawable;
use crate::res::R;

pub const IG_SIZE: PixelUnit = (20, 20);

lazy! {
    pub static CLIENT_INGREDIENT_REG: Registry<ClientIngredient> = Registry::new();
}

#[derive(Clone)]
pub struct ClientIngredient {
    id: usize,
    texture: Drawable
}

impl Registerable for ClientIngredient {
    type CreateInfo = Drawable;

    fn with_id(id: usize, info: Self::CreateInfo) -> Self {
        Self {
            id,
            texture: info,
        }
    }
}

pub fn register_ingredients() {
    CLIENT_INGREDIENT_REG.register(Drawable::Texture(R.texture.ingredient_stone));
}

pub struct LoadedClientIngredient {
    id: usize,
    texture: Drawable
}

impl LoadedClientIngredient {
    pub fn from_server(to_client: ToClientObject) -> Option<Self> {
        match to_client.source {
            ObjectSource::Vanilla => {
                let template = CLIENT_INGREDIENT_REG.create_object(to_client.id as usize)?;
                Some(LoadedClientIngredient {
                    id: template.id,
                    texture: template.texture,
                })
            }
            ObjectSource::Mod(m) => {
                //too bad mods are unsupported lol
                None
            }
        }
    }

    pub fn draw(&self, ctx: &mut impl WideRenderContext, view_area: &SimpleRect, at: TileUnit, tile_size: i32, y: f32) {
        drawutils::draw_in_world_size(ctx, view_area, at, IG_SIZE, Fill::Drawable(self.texture.clone(), Orientation::North), tile_size, y);
    }
}