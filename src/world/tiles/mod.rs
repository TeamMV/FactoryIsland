pub mod impls;

use crate::game::Game;
use crate::res::R;
use crate::world::terrain_tex_mapper::get_terrain_drawable;
use api::world::chunk::ToClientObject;
use api::world::tiles::pos::TilePos;
use api::world::tiles::Orientation;
use mvengine::color::RgbColor;
use mvengine::graphics::animation::GlobalAnimation;
use mvengine::graphics::tileset::TileSet;
use mvengine::math::vec::Vec4;
use mvengine::rendering::texture::Texture;
use mvengine::rendering::{InputVertex, Quad, RenderContext, Transform};
use mvengine::ui::context::UiResources;
use mvengine::ui::geometry::SimpleRect;
use mvengine::ui::res::OrMissingTexture;
use mvutils::unsafe_utils::Unsafe;
use mvutils::utils::TetrahedronOp;
use std::ops::Deref;
use bytebuffer::ByteBuffer;
use log::{debug, error, trace};
use mvengine::graphics::Drawable;
use mvutils::bytebuffer::ByteBufferExtras;
use api::registry::ObjectSource;
use crate::drawutils;
use crate::world::tiles::impls::{ClientStateTile, CustomDraw, CLIENT_TILE_REG};

pub trait TileDraw {
    fn draw(&self, ctx: &mut impl RenderContext, tile_size: i32, pos: &TilePos, orientation: Orientation, view_area: &SimpleRect, y: i32);
}

pub struct LoadedClientTile {
    pub id: usize,
    pub texture: ClientDrawable,
    pub orientation: Orientation,
    pub drawer: Option<CustomDraw>,
    pub state: Option<Box<dyn ClientStateTile>>
}

unsafe impl Send for LoadedClientTile {}
unsafe impl Sync for LoadedClientTile {}

impl LoadedClientTile {
    pub(crate) fn void() -> LoadedClientTile {
        Self {
            id: 0,
            texture: ClientDrawable::Texture(R.resolve_texture(R.mv.texture.missing).unwrap()),
            orientation: Orientation::North,
            drawer: None,
            state: None,
        }
    }
}

impl LoadedClientTile {
    pub fn from_server_tile(server_tile: ToClientObject, game: &Game, is_terrain: bool) -> Option<Self> {
        let orientation = server_tile.orientation;
        let state = server_tile.state;
        match &server_tile.source {
            ObjectSource::Vanilla => {
                let (drawable, drawer, state) = if is_terrain {
                    (get_terrain_drawable(server_tile.id as usize), None, None)
                } else {
                    if server_tile.id < 1 { return None; }
                    trace!("Loading tile with id: {}", server_tile.id);
                    if let Some(template) = CLIENT_TILE_REG.create_object(server_tile.id as usize - 1) {
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
                let tex = ClientDrawable::from_drawable(drawable, R.deref().deref());
                Some(Self {
                    id: server_tile.id as usize,
                    texture: tex,
                    orientation,
                    drawer,
                    state,
                })
            }
            ObjectSource::Mod(modid) => {
                let tex = if let Some(res) = game.client_resources.get(modid) {
                    //this is fine cuz u cannot unload mods at runtime
                    let res = unsafe { Unsafe::cast_lifetime(res) };
                    //again here //TODO
                    ClientDrawable::from_drawable(Drawable::missing(), res)
                } else {
                    ClientDrawable::Texture(R.resolve_texture(R.mv.texture.missing).unwrap())
                };
                Some(Self {
                    id: server_tile.id as usize,
                    texture: tex,
                    orientation,
                    drawer: None,
                    state: None,
                })
            }
        }
    }
}

impl TileDraw for LoadedClientTile {
    fn draw(&self, ctx: &mut impl RenderContext, tile_size: i32, pos: &TilePos, orientation: Orientation, view_area: &SimpleRect, y: i32) {
        if self.id != 0 {
            let (tex, uv) = self.texture.get_texture();
            let uv = orientation.apply(uv.as_uv());
            let tile_rect = SimpleRect::new(pos.raw.0 * tile_size, pos.raw.1 * tile_size, tile_size, tile_size);
            drawutils::tile_rect(ctx, uv, &tile_rect, view_area, tex, y);
        }
    }
}

pub enum ClientDrawable {
    Texture(&'static Texture),
    Animation(&'static GlobalAnimation<'static>),
    TileSet(&'static TileSet, usize)
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
            },
            Drawable::Color(_) => unimplemented!()
        }
    }
}