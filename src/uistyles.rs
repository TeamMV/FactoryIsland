use mvengine::ui::context::UiResources;
use mvengine::graphics::Drawable;
use mvengine_proc::resolve_resource;
use crate::res::R;
use mvengine::modify_style;
use mvengine::ui::styles::UiStyle;
use mvengine_proc::{multiline_str_into, style_expr, style_expr_empty};
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
        direction: vertical;
        margin: none;
        padding: none;
    });

    pub static FRAME: UiStyle = {
        let mut base = BG.clone();
        let s = multiline_str_into!(style_expr_empty,{
            position: relative;
            background.resource: color;
            background.color: @R.color/ui_bg;
            width: 50%;
            height: 80%;
            child_align_x: start;
            child_align_y: start;
            direction: vertical;
            padding: 1cm;
        });
        base.merge_at_set_of(&s);
        base
    };

    pub static PRESET: UiStyle = multiline_str_into!(style_expr,{
        background.color: @R.color/ui_bg;
        detail.color: @R.color/ui_highlight;
        detail.shape: @R.geometry/tick;
        text.color: @R.color/ui_highlight;
        width: @R.dimension/ui_widget_width;
        height: @R.dimension/ui_widget_height;
        border.resource: none;
        text.size: 1cm;
        text.align_x: middle;
        text.align_y: middle;
        padding: none;
        overflow_y: never;
        overflow_x: never;
    });

    pub static EDIT_PRESET: UiStyle = {
        let m = multiline_str_into!(style_expr_empty, {
            padding: 0.2cm;
        });
        let mut base = PRESET.clone();
        base.merge_at_set_of(&m);
        base
    };

    pub static CHECKBOX_PRESET: UiStyle = {
        let m = multiline_str_into!(style_expr_empty, {
            text.align_x: start;
        });
        let mut base = PRESET.clone();
        base.merge_at_set_of(&m);
        base
    };

    pub static CLEAR_PRESET: UiStyle = {
        let mut base = PRESET.clone();
        base.merge_at_set_of(&CLEAR);
        base
    };

    pub static CLEAR: UiStyle = multiline_str_into!(style_expr_empty,{
        background.resource: none;
        border.resource: none;
    });
}