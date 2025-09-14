use mvengine::ui::context::UiResources;
use crate::res::R;
use mvengine_proc::resolve_resource;
use mvengine_proc::style_expr;
use mvengine::modify_style;
use mvengine::ui::styles::UiStyle;
use mvengine_proc::style_expr_empty;
use mvengine_proc::multiline_str_into;
use mvutils::lazy;

lazy! {    
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
        margin: 0.5bf, 0;
        overflow_y: never;
        overflow_x: never;
        scrollbar.size: 2mm;
        scrollbar.knob.color: @R.color/ui_highlight;
        scrollbar.track.color: @R.color/ui_bg;
    });

    pub static PRESET_SMALL: UiStyle = {
        let m = multiline_str_into!(style_expr_empty, {
            width: @R.dimension/ui_widget_width_small;
        });
        let mut base = PRESET.clone();
        base.merge_at_set_of(&m);
        base
    };

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

    pub static FRAME: UiStyle = {
        let m = multiline_str_into!(style_expr_empty, {
            width: 100%;
            height: 100%;
            direction: vertical;
            child_align_x: middle;
            child_align_y: start;
            padding: 1cm;
            margin: none;
        });
        let mut base = CLEAR.clone();
        base.merge_at_set_of(&m);
        base
    };

    pub static H_LAYOUT: UiStyle = {
        let m = multiline_str_into!(style_expr_empty, {
            direction: horizontal;
            padding: none;
            margin: none;
            width: 100%;
        });
        let mut base = CLEAR.clone();
        base.merge_at_set_of(&m);
        base
    };

    pub static V_LAYOUT: UiStyle = {
        let m = multiline_str_into!(style_expr_empty, {
            direction: vertical;
            padding: none;
            margin: none;
        });
        let mut base = CLEAR.clone();
        base.merge_at_set_of(&m);
        base
    };
}