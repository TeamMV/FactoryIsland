use crate::game::Game;
use crate::res::R;
use crate::uistyles;
use crate::uistyles::{CLEAR_PRESET, H_LAYOUT};
use api::ingredients::IngredientStack;
use api::inventory::ItemAction;
use api::meta::{Meta, MetaField};
use api::registry::ingredients::INGREDIENT_REGISTRY;
use mvengine::ui::context::UiContext;
use mvengine::ui::context::UiResources;
use mvengine::ui::elements::prelude::*;
use mvengine::ui::elements::Element;
use mvengine::window::Window;
use mvengine::{expect_element_by_id, modify_style};
use mvengine_proc::resolve_resource;
use mvengine_proc::{style_expr_empty, ui};
use mvutils::lazy;
use mvutils::state::State;

pub struct IngredientDisplay {
    item_in_question: IngredientStack,
    droppable: bool,
    transferable: bool,
    transfer_action: ItemAction,
    inv_idx: u64,
    item_idx: u64,

    // mutable shit
    selected_amt: State<u64>,

    //ui shit
    container: Element,
}

lazy! {
    static EMPTY_META: Meta = Meta::new();
}

impl IngredientDisplay {
    pub fn new(
        ctx: UiContext,
        game: &Game,
        item_idx: u64,
        inv_idx: u64,
        transfer_action: ItemAction,
        transferable: bool,
        droppable: bool,
        item_in_question: IngredientStack,
    ) -> Self {
        let container = Self::create_ui(ctx, &item_in_question, game, transferable, droppable);

        Self {
            item_in_question,
            droppable,
            transferable,
            transfer_action,
            inv_idx,
            item_idx,
            selected_amt: State::new(1),
            container,
        }
    }

    fn create_ui(
        ctx: UiContext,
        item_in_question: &IngredientStack,
        game: &Game,
        transfer: bool,
        droppable: bool,
    ) -> Element {
        let mut meta_cont_style = uistyles::V_LAYOUT.clone();
        //idk max what to put here
        meta_cont_style.merge_at_set_of(&style_expr_empty!("height.max: 6cm;"));

        let s_meta = match INGREDIENT_REGISTRY.reference_object(item_in_question.ingredient) {
            Some(x) => x.static_meta(),
            None => &EMPTY_META,
        };

        let mut outer_style = uistyles::V_LAYOUT.clone();
        outer_style.merge_at_set_of(&style_expr_empty!(
            "background.resource: color; background.color: @R.color/inv_bg;"
        ));

        println!("create_ui");

        let mut ui = ui! {
            <Ui context={ctx.clone()}>
                <Div style={outer_style.clone()}>
                    //top image and stuff

                    //meta fields
                    <Div id="meta_container" style={meta_cont_style}>
                        {
                            item_in_question.meta.iter()
                                .chain(s_meta.iter())
                                .map(|(_, x)| {
                                    Self::create_meta_field_ui(ctx.clone(), x, game)
                                })
                        }
                    </Div>

                    //actions
                </Div>
            </Ui>
        };

        ui
    }

    fn create_meta_field_ui(ctx: UiContext, meta_field: &MetaField, game: &Game) -> Element {
        let display_name = game
            .language
            .maybe_lookup(&format!("ingredient.display.meta.{}", meta_field.key));
        let entry_value = meta_field.to_string();

        println!("create field: {display_name} = {entry_value}");

        let mut entry_style = CLEAR_PRESET.clone();
        entry_style.merge_at_set_of(&style_expr_empty!(
            "margin: none; overflow_x: never; padding.bottom: 1bf"
        ));

        let mut div_style = H_LAYOUT.clone();
        div_style.merge_at_set_of(&style_expr_empty!("width: 50%"));
        ui! {
            <Ui context={ctx}>
                <Div style={uistyles::H_LAYOUT_MAX.clone()}>
                    <Div style={div_style.clone()}><Button style={entry_style.clone()}>{display_name}</Button></Div>
                    <Div style={div_style.clone()}><Button style={entry_style.clone()}>{entry_value}</Button></Div>
                </Div>
            </Ui>
        }
    }

    pub fn check_events(&mut self, window: &mut Window, game: &Game) {}

    pub fn container(&self) -> &Element {
        &self.container
    }
}
