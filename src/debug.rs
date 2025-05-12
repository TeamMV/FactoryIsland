use mvengine::color::RgbColor;
use mvengine::rendering::RenderContext;
use mvengine::ui::rendering::adaptive::AdaptiveFill;
use crate::drawutils;

pub fn debug_rect(ctx: &mut impl RenderContext, x: i32, y: i32, w: i32, h: i32, color: RgbColor) {
    drawutils::rect(ctx, x, y, w, h, AdaptiveFill::Color(color), 10.0)
}