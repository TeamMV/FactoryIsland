pub mod resources;

use crate::game::Game;
use crate::res::R;
use crate::world::terrain_tex_mapper::get_terrain_drawable;
use crate::world::tile_tex_mapper::get_tile_drawable;
use api::world::chunk::ToClientObject;
use api::world::tiles::pos::TilePos;
use api::world::tiles::terrain::ObjectSource;
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
use mvengine::graphics::Drawable;

pub trait TileDraw {
    fn draw(&self, ctx: &mut impl RenderContext, tile_size: i32, pos: &TilePos, orientation: Orientation, view_area: &SimpleRect, y: i32);

    fn static_rect(ctx: &mut impl RenderContext, uv: [(f32, f32); 4], tile_rect: &SimpleRect, view_area: &SimpleRect, tex: &Texture, y: i32) {
        let controller = ctx.controller();

        let x1 = tile_rect.x as f32 - view_area.x as f32;
        let x2 = tile_rect.x as f32 + tile_rect.width as f32 - view_area.x as f32;
        let y1 = tile_rect.y as f32 - view_area.y as f32;
        let y2 = tile_rect.y as f32 + tile_rect.height as f32 - view_area.y as f32;

        //replace with biome tint in future
        let tint = None.unwrap_or(RgbColor::transparent());

        controller.push_quad(Quad {
            points: [
                InputVertex {
                    transform: Transform::new().translate_self(x1, y1),
                    pos: (0.0, 0.0, y as f32),
                    color: tint.as_vec4(),
                    uv: uv[0],
                    texture: tex.id,
                    has_texture: 1.0,
                },
                InputVertex {
                    transform: Transform::new().translate_self(x1, y2),
                    pos: (0.0, 0.0, y as f32),
                    color: tint.as_vec4(),
                    uv: uv[1],
                    texture: tex.id,
                    has_texture: 1.0,
                },
                InputVertex {
                    transform: Transform::new().translate_self(x2, y2),
                    pos: (0.0, 0.0, y as f32),
                    color: tint.as_vec4(),
                    uv: uv[2],
                    texture: tex.id,
                    has_texture: 1.0,
                },
                InputVertex {
                    transform: Transform::new().translate_self(x2, y1),
                    pos: (0.0, 0.0, y as f32),
                    color: tint.as_vec4(),
                    uv: uv[3],
                    texture: tex.id,
                    has_texture: 1.0,
                }
            ],
        });
    }
}

pub struct ClientTile {
    pub id: usize,
    pub texture: ClientDrawable,
    pub orientation: Orientation
}

impl ClientTile {
    pub(crate) fn void() -> ClientTile {
        Self {
            id: 0,
            texture: ClientDrawable::Texture(R.resolve_texture(R.mv.texture.missing).unwrap()),
            orientation: Orientation::North,
        }
    }
}

impl ClientTile {
    pub fn from_server_tile(server_tile: ToClientObject, game: &Game, is_terrain: bool) -> Self {
        let orientation = server_tile.orientation;
        let state = server_tile.state;
        match &server_tile.source {
            ObjectSource::Vanilla => {
                let drawable = is_terrain.yn(
                    get_terrain_drawable(server_tile.id as usize),
                    get_tile_drawable(server_tile.id as usize, state)
                );
                let tex = ClientDrawable::from_drawable(drawable, R.deref().deref());
                Self {
                    id: server_tile.id as usize,
                    texture: tex,
                    orientation,
                }
            }
            ObjectSource::Mod(modid, mapper) => {
                let tex = if let Some(res) = game.client_resources.get(modid) {
                    //this is fine cuz u cannot unload mods at runtime
                    let res = unsafe { Unsafe::cast_static(res) };
                    ClientDrawable::from_drawable(mapper.map(state), res)
                } else {
                    ClientDrawable::Texture(R.resolve_texture(R.mv.texture.missing).unwrap())
                };
                Self {
                    id: server_tile.id as usize,
                    texture: tex,
                    orientation,
                }
            }
        }
    }
}

impl TileDraw for ClientTile {
    fn draw(&self, ctx: &mut impl RenderContext, tile_size: i32, pos: &TilePos, orientation: Orientation, view_area: &SimpleRect, y: i32) {
        if self.id != 0 {
            let (tex, uv) = self.texture.get_texture();
            let uv = orientation.apply(uv.as_uv());
            let tile_rect = SimpleRect::new(pos.raw.0 * tile_size, pos.raw.1 * tile_size, tile_size, tile_size);
            Self::static_rect(ctx, uv, &tile_rect, view_area, tex, y);
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