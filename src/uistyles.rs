use crate::res::R;
use mvengine::graphics::Drawable;
use mvengine::modify_style;
use mvengine::ui::context::UiResources;
use mvengine::ui::styles::UiStyle;
use mvengine_proc::resolve_resource;
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
        overflow_x: never;
        overflow_y: never;
    });

    pub static OUTER_FRAME: UiStyle = {
        let m = multiline_str_into!(style_expr_empty, {
            width: 100%;
            height: 100%;
            padding: 2.5cm;
            overflow_x: never;
            overflow_y: never;
            child_align_x: middle;
            child_align_y: middle;
        });
        let mut base = CLEAR.clone();
        base.merge_at_set_of(&m);
        base
    };

    pub static FRAME: UiStyle = {
        let mut base = BG.clone();
        let s = multiline_str_into!(style_expr_empty,{
            position: relative;
            background.resource: color;
            background.color: @R.color/ui_bg;
            width: 100%;
            height: 100%;
            child_align_x: start;
            child_align_y: start;
            direction: vertical;
            padding: 1cm;
            overflow_x: normal;
            overflow_y: normal;
        });
        base.merge_at_set_of(&s);
        base
    };

    pub static PRESET: UiStyle = multiline_str_into!(style_expr,{
        background.color: @R.color/ui_bg;
        hover.background.color: @R.color/ui_bg_hover;
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

    pub static SLIDER_PRESET: UiStyle = {
        let m = multiline_str_into!(style_expr_empty, {
            detail.shape: @R.geometry/knob;
            detail.resource: color;
            detail.color: @R.color/ui_highlight;
            text.color: white;
            padding: 3mm;
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

    pub static SLOT_OUTER_STYLE: UiStyle = multiline_str_into!(style_expr,{
        background.resource: color;
        hover.background.color: @R.color/inv_slot_hover;
        border.resource: none;
        width: 1.5cm;
        height: 1.5cm;
        padding: 2mm;
        margin: 1.5mm;
        overflow_x: never;
        overflow_y: never;
    });

    pub static SLOT_INNER_STYLE: UiStyle = multiline_str_into!(style_expr,{
        background.resource: texture;
        border.resource: none;
        width: 100%;
        height: 100%;
        padding: none;
        margin: none;
        overflow_x: never;
        overflow_y: never;
    });

    pub static INVENTORY_WRAPPER_STYLE: UiStyle = multiline_str_into!(style_expr,{
        background.resource: none;
        border.resource: none;
        direction: vertical;
        child_align_x: middle;
    });

    pub static INVENTORY_STYLE: UiStyle = multiline_str_into!(style_expr,{
        background.resource: color;
        background.color: @R.color/inv_bg;
        border.resource: none;
        padding: none;
        direction: vertical;
        margin: 1cm;
        height: 30%;
    });
}
