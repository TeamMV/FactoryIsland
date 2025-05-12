use crate::res::R;
use api::world::{resolve_unit, TileUnit};
use api::world::tiles::pos::TilePos;
use mvengine::rendering::{Quad, RenderContext};
use mvengine::ui::geometry::SimpleRect;
use mvengine::ui::rendering::adaptive::AdaptiveFill;
use mvengine::ui::rendering::ctx;
use std::ops::Deref;
use crate::drawutils;

pub fn rect(ctx: &mut impl RenderContext, x: i32, y: i32, w: i32, h: i32, fill: AdaptiveFill, z: f32) {
    let c = ctx.controller();
    let rect = match fill {
        AdaptiveFill::Color(col) => {
            ctx::rectangle()
                .xywh(x, y, w, h)
                .color(col)
                .z(z)
                .create()
        }
        AdaptiveFill::Drawable(drawable) => {
            let (texture, uv) = drawable.get_texture_or_default(R.deref().deref());
            ctx::rectangle()
                .xywh(x, y, w, h)
                .texture(ctx::texture()
                    .source(Some(texture.clone()))
                    .uv(uv)
                )
                .z(1.0)
                .create()
        }
    };

    c.push_quad(Quad {
        points: [
            rect.triangles[0].points[0].clone(),
            rect.triangles[0].points[1].clone(),
            rect.triangles[0].points[2].clone(),
            rect.triangles[1].points[2].clone(),
        ],
    });
}

pub fn draw_in_world(ctx: &mut impl RenderContext, view_area: &SimpleRect, pos: TileUnit, size: TileUnit, fill: AdaptiveFill, tile_size: i32, y: f32) {
    let pos_px = resolve_unit(pos, tile_size);
    let size_px = resolve_unit(size, tile_size);
    let rect = SimpleRect::new(pos_px.0, pos_px.1, size_px.0, size_px.1);
    if view_area.intersects(&rect) {
        drawutils::rect(ctx, rect.x - view_area.x, rect.y - view_area.y, rect.width, rect.height, fill, y);
    }
}

pub fn draw_in_world_tile(ctx: &mut impl RenderContext, view_area: &SimpleRect, pos: TilePos, fill: AdaptiveFill, tile_size: i32, y: f32) {
    let x = pos.raw.0 * tile_size;
    let z = pos.raw.1 * tile_size;
    let rect = SimpleRect::new(x, z, tile_size, tile_size);
    if view_area.intersects(&rect) {
        drawutils::rect(ctx, rect.x - view_area.x, rect.y - view_area.y, rect.width, rect.height, fill, y);
    }
}