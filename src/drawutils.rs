use crate::drawutils;
use crate::res::R;
use api::world::tiles::pos::TilePos;
use api::world::{resolve_unit, TileUnit};
use mvengine::rendering::RenderContext;
use mvengine::ui::geometry::shape::{shapes, VertexStream};
use mvengine::ui::geometry::{Rect, SimpleRect};
use mvengine::ui::rendering::adaptive::AdaptiveFill;
use mvengine::ui::rendering::WideRenderContext;
use std::ops::Deref;

pub fn rect(ctx: &mut impl WideRenderContext, x: i32, y: i32, w: i32, h: i32, fill: AdaptiveFill, z: f32) {
    match fill {
        AdaptiveFill::Color(col) => {
            let rect = shapes::rectangle0(x, y, w, h);
            rect.draw(ctx, |v| {
                v.color = col.as_vec4();
                v.pos.2 = z;
            });
        }
        AdaptiveFill::Drawable(drawable) => {
            let prev_z = ctx.next_z();
            ctx.set_z(z);
            let crop = SimpleRect::new(ctx.x(), ctx.y(), ctx.width(), ctx.height());
            drawable.draw(ctx, Rect::simple(x, y, w, h), R.deref().deref(), &crop);
            ctx.set_z(prev_z);
        }
    };
}

pub fn draw_in_world(ctx: &mut impl WideRenderContext, view_area: &SimpleRect, pos: TileUnit, size: TileUnit, fill: AdaptiveFill, tile_size: i32, y: f32) {
    let pos_px = resolve_unit(pos, tile_size);
    let size_px = resolve_unit(size, tile_size);
    let rect = SimpleRect::new(pos_px.0, pos_px.1, size_px.0, size_px.1);
    if view_area.intersects(&rect) {
        drawutils::rect(ctx, rect.x - view_area.x, rect.y - view_area.y, rect.width, rect.height, fill, y);
    }
}

pub fn draw_in_world_tile(ctx: &mut impl WideRenderContext, view_area: &SimpleRect, pos: TilePos, fill: AdaptiveFill, tile_size: i32, y: f32) {
    let x = pos.raw.0 * tile_size;
    let z = pos.raw.1 * tile_size;
    let rect = SimpleRect::new(x, z, tile_size, tile_size);
    if view_area.intersects(&rect) {
        drawutils::rect(ctx, rect.x - view_area.x, rect.y - view_area.y, rect.width, rect.height, fill, y);
    }
}