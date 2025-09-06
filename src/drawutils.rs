use crate::drawutils;
use crate::res::R;
use api::world::tiles::pos::TilePos;
use api::world::{resolve_unit, PixelUnit, TileUnit};
use mvengine::rendering::{InputVertex, Quad, RenderContext, Transform};
use mvengine::ui::geometry::shape::{shapes, VertexStream};
use mvengine::ui::geometry::{Rect, SimpleRect};
use mvengine::ui::rendering::adaptive::AdaptiveFill;
use mvengine::ui::rendering::WideRenderContext;
use std::ops::Deref;
use mvengine::color::RgbColor;
use mvengine::graphics::Drawable;
use mvengine::rendering::texture::Texture;
use mvengine::ui;
use api::world::tiles::Orientation;
use crate::world::tiles::ClientDrawable;

pub enum Fill {
    Color(RgbColor),
    Drawable(Drawable, Orientation),
    ClientDrawable(ClientDrawable, Orientation),
}

pub fn tile_rect(ctx: &mut impl RenderContext, uv: [(f32, f32); 4], tile_rect: &SimpleRect, view_area: &SimpleRect, tex: &Texture, y: i32) {
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
                transform: Transform::new(),
                pos: (x1, y1, y as f32),
                color: tint.as_vec4(),
                uv: uv[0],
                texture: tex.id,
                has_texture: 1.0,
            },
            InputVertex {
                transform: Transform::new(),
                pos: (x1, y2, y as f32),
                color: tint.as_vec4(),
                uv: uv[1],
                texture: tex.id,
                has_texture: 1.0,
            },
            InputVertex {
                transform: Transform::new(),
                pos: (x2, y2, y as f32),
                color: tint.as_vec4(),
                uv: uv[2],
                texture: tex.id,
                has_texture: 1.0,
            },
            InputVertex {
                transform: Transform::new(),
                pos: (x2, y1, y as f32),
                color: tint.as_vec4(),
                uv: uv[3],
                texture: tex.id,
                has_texture: 1.0,
            }
        ],
    });
}

pub fn rect(ctx: &mut impl WideRenderContext, x: i32, y: i32, w: i32, h: i32, fill: Fill, z: f32) {
    match fill {
        Fill::Color(col) => {
            let rect = shapes::rectangle0(x, y, w, h);
            rect.draw(ctx, |v| {
                v.color = col.as_vec4();
                v.pos.2 = z;
            });
        }
        Fill::Drawable(drawable, orientation) => {
            let (tex, uv) = drawable.get_texture_or_default(R.deref().deref());
            let uv = Texture::get_uv_inner_static(uv);
            let uv = orientation.apply(uv);

            let mut v1 = shapes::vertex1(x, y, tex.id, uv[0]);
            let mut v2 = shapes::vertex1(x, y + h, tex.id, uv[1]);
            let mut v3 = shapes::vertex1(x + w, y + h, tex.id, uv[2]);
            let mut v4 = shapes::vertex1(x + w, y, tex.id, uv[3]);

            for v in [&mut v1, &mut v2, &mut v3, &mut v4] {
                v.pos.2 = z;
            }

            ctx.controller().push_quad(Quad {
                points: [v1, v2, v3, v4],
            });
        }
        Fill::ClientDrawable(drawable, orientation) => {
            let (tex, uv) = drawable.get_texture();
            let uv = Texture::get_uv_inner_static(uv);
            let uv = orientation.apply(uv);

            let mut v1 = shapes::vertex1(x, y, tex.id, uv[0]);
            let mut v2 = shapes::vertex1(x, y + h, tex.id, uv[1]);
            let mut v3 = shapes::vertex1(x + w, y + h, tex.id, uv[2]);
            let mut v4 = shapes::vertex1(x + w, y, tex.id, uv[3]);

            for v in [&mut v1, &mut v2, &mut v3, &mut v4] {
                v.pos.2 = z;
            }

            ctx.controller().push_quad(Quad {
                points: [v1, v2, v3, v4],
            });
        }
    };
}

pub fn draw_in_world(ctx: &mut impl WideRenderContext, view_area: &SimpleRect, pos: TileUnit, size: TileUnit, fill: Fill, tile_size: i32, y: f32) {
    let pos_px = resolve_unit(pos, tile_size);
    let size_px = resolve_unit(size, tile_size);
    let rect = SimpleRect::new(pos_px.0, pos_px.1, size_px.0, size_px.1);
    if view_area.intersects(&rect) {
        drawutils::rect(ctx, rect.x - view_area.x, rect.y - view_area.y, rect.width, rect.height, fill, y);
    }
}

pub fn draw_in_world_tile(ctx: &mut impl WideRenderContext, view_area: &SimpleRect, pos: TilePos, fill: Fill, tile_size: i32, y: f32) {
    let x = pos.raw.0 * tile_size;
    let z = pos.raw.1 * tile_size;
    let rect = SimpleRect::new(x, z, tile_size, tile_size);
    if view_area.intersects(&rect) {
        drawutils::rect(ctx, rect.x - view_area.x, rect.y - view_area.y, rect.width, rect.height, fill, y);
    }
}

pub fn draw_in_world_size(ctx: &mut impl WideRenderContext, view_area: &SimpleRect, pos: TileUnit, size: PixelUnit, fill: Fill, tile_size: i32, y: f32) {
    let pos_px = resolve_unit(pos, tile_size);
    let size_px = size;
    let rect = SimpleRect::new(pos_px.0, pos_px.1, size_px.0, size_px.1);
    if view_area.intersects(&rect) {
        drawutils::rect(ctx, rect.x - view_area.x, rect.y - view_area.y, rect.width, rect.height, fill, y);
    }
}

pub fn get_screen_pos(view_area: &SimpleRect, pos: TileUnit, tile_size: i32) -> PixelUnit {
    let (x, y) = resolve_unit(pos, tile_size);
    (x - view_area.x, y - view_area.y)
}