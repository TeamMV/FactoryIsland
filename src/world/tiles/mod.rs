pub mod impls;

use crate::drawutils;
use crate::drawutils::Fill;
use crate::game::Game;
use crate::res::R;
use crate::world::terrain_tex_mapper::get_terrain_drawable;
use crate::world::tiles::impls::{ClientStateTile, CustomDraw, CLIENT_TILE_REG};
use api::world::chunk::ToClientObject;
use api::world::tiles::pos::TilePos;
use api::world::tiles::{Orientation, TileKind};
use bytebuffer::ByteBuffer;
use log::{debug, error, trace};
use mvengine::color::RgbColor;
use mvengine::graphics::animation::GlobalAnimation;
use mvengine::graphics::tileset::TileSet;
use mvengine::graphics::Drawable;
use mvengine::math::vec::Vec4;
use mvengine::rendering::texture::Texture;
use mvengine::rendering::{InputVertex, Quad, RenderContext, Transform};
use mvengine::ui::context::UiResources;
use mvengine::ui::geometry::SimpleRect;
use mvengine::ui::rendering::WideRenderContext;
use mvengine::ui::res::OrMissingTexture;
use mvutils::bytebuffer::ByteBufferExtras;
use mvutils::unsafe_utils::Unsafe;
use mvutils::utils::TetrahedronOp;
use std::ops::Deref;

pub trait TileDraw {
    fn draw(
        &self,
        ctx: &mut impl WideRenderContext,
        tile_size: i32,
        pos: &TilePos,
        orientation: Orientation,
        view_area: &SimpleRect,
        y: i32,
    );
}

pub struct LoadedClientTile {
    pub id: TileKind,
    pub texture: Drawable,
    pub orientation: Orientation,
    pub drawer: Option<CustomDraw>,
    pub state: Option<Box<dyn ClientStateTile>>,
}

unsafe impl Send for LoadedClientTile {}
unsafe impl Sync for LoadedClientTile {}

impl LoadedClientTile {
    pub(crate) fn void() -> LoadedClientTile {
        Self {
            id: 0,
            texture: Drawable::missing(),
            orientation: Orientation::North,
            drawer: None,
            state: None,
        }
    }
}

impl LoadedClientTile {
    pub fn from_server_tile(
        server_tile: ToClientObject,
        is_terrain: bool,
    ) -> Option<Self> {
        let orientation = server_tile.orientation;
        let state = server_tile.state;

        let (drawable, drawer, state) = if is_terrain {
            (get_terrain_drawable(server_tile.id as usize), None, None)
        } else {
            if server_tile.id < 1 {
                return None;
            }
            if let Some(template) =
                CLIENT_TILE_REG.create_object(server_tile.id as usize - 1)
            {
                let (drawable, state) = if let Some(mut st) = template.state {
                    if !state.is_empty() {
                        let mut buf = ByteBuffer::from_vec_le(state);
                        if let Err(e) = st.load_from_server(&mut buf) {
                            error!("Error when loading client state: {e}");
                            (st.get_drawable(), Some(st))
                        } else {
                            (st.get_drawable(), Some(st))
                        }
                    } else {
                        (st.get_drawable(), None)
                    }
                } else {
                    (template.base, None)
                };
                let drawer = if let Some(drawer) = template.drawer {
                    Some(drawer)
                } else {
                    None
                };
                (drawable, drawer, state)
            } else {
                (Drawable::missing(), None, None)
            }
        };
        let tex = drawable;
        Some(Self {
            id: server_tile.id,
            texture: tex,
            orientation,
            drawer,
            state,
        })
    }

    pub fn new_ghost(id: TileKind, orientation: Orientation) -> Self {
        if let Some(template) = CLIENT_TILE_REG.create_object(id as usize) {
            Self {
                id,
                texture: template.base,
                orientation,
                drawer: template.drawer,
                state: template.state,
            }
        } else {
            Self::void()
        }
    }
}

impl TileDraw for LoadedClientTile {
    fn draw(
        &self,
        ctx: &mut impl WideRenderContext,
        tile_size: i32,
        pos: &TilePos,
        orientation: Orientation,
        view_area: &SimpleRect,
        y: i32,
    ) {
        if self.id != 0 {
            drawutils::draw_in_world_tile(
                ctx,
                view_area,
                pos.clone(),
                Fill::Drawable(self.texture.clone(), orientation),
                tile_size,
                y as f32,
            );
        }
    }
}

#[derive(Clone)]
pub enum ClientDrawable {
    Texture(&'static Texture),
    Animation(&'static GlobalAnimation<'static>),
    TileSet(&'static TileSet, usize),
}

impl ClientDrawable {
    pub fn get_texture(&self) -> (&Texture, Vec4) {
        match self {
            ClientDrawable::Texture(t) => (t, Vec4::default_uv()),
            ClientDrawable::Animation(a) => a.get_current(),
            ClientDrawable::TileSet(t, i) => {
                if let Some(r) = t.get_tile(*i) {
                    r
                } else {
                    let tex = R.resolve_texture(R.mv.texture.missing).unwrap();
                    (tex, Vec4::default_uv())
                }
            }
        }
    }

    pub fn from_drawable(drawable: Drawable, res: &'static impl UiResources) -> Self {
        match drawable {
            Drawable::Texture(t) => {
                ClientDrawable::Texture(res.resolve_texture(t).or_missing_texture())
            }
            Drawable::Animation(a) => {
                if let Some(anim) = res.resolve_animation(a) {
                    ClientDrawable::Animation(anim)
                } else {
                    ClientDrawable::Texture(R.resolve_texture(R.mv.texture.missing).unwrap())
                }
            }
            Drawable::TileSet(t, i) => {
                if let Some(tileset) = res.resolve_tileset(t) {
                    ClientDrawable::TileSet(tileset, i)
                } else {
                    ClientDrawable::Texture(R.resolve_texture(R.mv.texture.missing).unwrap())
                }
            }
            Drawable::Color(_) => unimplemented!(),
        }
    }
}
