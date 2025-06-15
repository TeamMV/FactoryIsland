use mvengine::ui::context::UiResources;
use mvengine::graphics::Drawable;
use mvengine_proc::resolve_resource;
use crate::res::R;
use mvengine::modify_style;
use mvengine::ui::styles::UiStyle;
use mvengine_proc::{multiline_str_into, style_expr};
use mvutils::lazy;

lazy! {
    pub static BG: UiStyle = multiline_str_into!(style_expr,{
        position: absolute;
        width: 100%;
        height: 100%;
        child_align_x: middle;
        child_align_y: middle;
        background.resource: texture;
        background.texture: @R.drawable/bg;
    });
}