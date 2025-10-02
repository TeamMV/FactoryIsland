use crate::game::Game;
use crate::ingredients::CLIENT_INGREDIENT_REG;
use crate::res::R;
use crate::ui::display::ingredient::IngredientDisplay;
use crate::uistyles;
use api::ingredients::IngredientStack;
use api::inventory::{InventoryData, InventoryOwner, ItemAction};
use api::server::packets::inventory::InventoryDataPacket;
use mvengine::graphics::Drawable;
use mvengine::modify_style;
use mvengine::ui::context::UiContext;
use mvengine::ui::context::UiResources;
use mvengine::ui::elements::prelude::*;
use mvengine::ui::elements::Element;
use mvengine::ui::layouts::uniqueselect::UniqueSelectLayout;
use mvengine::ui::styles::Parseable;
use mvengine::window::Window;
use mvengine_proc::style_expr;
use mvengine_proc::{resolve_resource, style_expr_empty, ui};
use mvutils::utils::TetrahedronOp;
use std::slice::Iter;

pub struct InventoryDisplay {
    allowed_actions: ItemAction,
    data: InventoryData,
    owner: InventoryOwner,
    element: Option<Element>,
    player_inv: Option<Box<InventoryDisplay>>,
    all_buttons: Vec<Element>,
    is_nested: bool,

    selection: Option<UniqueSelectLayout>,
    item_display: Option<IngredientDisplay>,
    item_display_container: Element,
}
// в В т Т ь Ь ч Ч к К п П р Р д Д ж Ж ф Ф ы Ы у У н Н г Г ш Ш я Я м М и И б Б ю Ю э Э х Х з З

impl InventoryDisplay {
    pub fn new(mut data: InventoryDataPacket, ctx: UiContext) -> Self {
        let item_display_container = ui! {
            <Ui context={ctx}>
                <Div style="background: none; border: none; position: absolute; x: 100%; y: 100%; width: 7cm; origin: top_right; direction: vertical; margin: none; padding: 1cm;"/>
            </Ui>
        };

        let player_inv = data.player_inventory.map(|inv_data| {
            Box::new(InventoryDisplay {
                allowed_actions: data.item_actions,
                data: inv_data,
                owner: InventoryOwner::Player,
                element: None,
                player_inv: None,
                all_buttons: vec![],
                is_nested: true,
                selection: None,
                item_display: None,
                item_display_container: item_display_container.clone(),
            })
        });
        InventoryDisplay {
            allowed_actions: data.item_actions,
            data: data.data,
            owner: data.owner,
            element: None,
            player_inv,
            all_buttons: vec![],
            is_nested: false,
            selection: None,
            item_display: None,
            item_display_container,
        }
    }

    pub fn create_ui(&mut self, ctx: UiContext) {
        let mut div = ui! {
            <Ui context={ctx.clone()}>
                <Div style={uistyles::INVENTORY_WRAPPER_STYLE.clone()}/>
            </Ui>
        };

        let e = div.get_mut();
        e.add_child(self.generate_ui(ctx.clone()).to_child());

        if let Some(player_inv) = &mut self.player_inv {
            e.add_child(player_inv.generate_ui(ctx).to_child());
        }

        self.element = Some(div);
    }

    fn generate_ui(&mut self, ctx: UiContext) -> Element {
        let mut div: Element = ui! {
            <Ui context={ctx.clone()}>
                <Div style={uistyles::INVENTORY_STYLE.clone()}/>
            </Ui>
        };
        let e = div.get_mut();

        self.all_buttons.clear();
        let mut iter = self.data.stacks.iter();
        while let Some(row) = Self::create_slot_row(
            &mut iter,
            ctx.clone(),
            self.data.width as usize,
            &mut self.all_buttons,
        ) {
            e.add_child(row.to_child());
        }

        let sel = UniqueSelectLayout::new(self.all_buttons.clone());
        self.selection = Some(sel);

        div
    }

    fn check_events(&mut self, window: &mut Window, game: &Game) {
        let show_drop = self.allowed_actions.can_drop();
        let show_transfer = self.is_nested.yn(
            self.allowed_actions.can_transfer_from_player(),
            self.allowed_actions.can_transfer_to_player(),
        );
        let transfer_type = self.is_nested.yn(
            ItemAction::TRANSFER_FROM_PLAYER,
            ItemAction::TRANSFER_TO_PLAYER,
        );

        if let Some(sel) = &mut self.selection {
            if let Some(idx) = sel.check_events() {
                let stack = &self.data.stacks[idx];

                println!("opened");

                let id = IngredientDisplay::new(
                    window.ui().context(),
                    game,
                    idx as u64,
                    self.data.id,
                    transfer_type,
                    show_transfer,
                    show_drop,
                    stack.clone(),
                );

                if let Some(_) = self.item_display.take() {
                    self.item_display_container.remove_all_children();
                }

                let container = id.container().clone();
                self.item_display_container.add_child(container.to_child());
                self.item_display = Some(id);
            }
        }

        if let Some(inner) = &mut self.player_inv {
            inner.check_events(window, game);
        }
    }

    fn create_slot(stack: &IngredientStack, ctx: UiContext) -> Element {
        let id = stack.ingredient;
        println!("ingredient id: {id}");
        let no_border = style_expr_empty!("border.resource: none;");
        let (tex, color, border_style) = if let Some(ing) = CLIENT_INGREDIENT_REG.create_object(id)
        {
            let (color, border) = if let Some(color) = ing.override_bg {
                (
                    color,
                    style_expr_empty!("border.resource: color; border.color: @R.color/inv_slot_bg"),
                )
            } else {
                (
                    resolve_resource!("@R.color/inv_slot_bg").unwrap().clone(),
                    no_border.clone(),
                )
            };
            (ing.texture.clone(), color, border)
        } else {
            (
                Drawable::missing(),
                resolve_resource!("@R.color/inv_slot_bg").unwrap().clone(),
                no_border,
            )
        };

        let mut style = uistyles::SLOT_INNER_STYLE.clone();
        style.merge_at_set_of(&style_expr_empty!("background.texture: {tex.clone()};"));
        let mut outer_style = uistyles::SLOT_OUTER_STYLE.clone();
        outer_style.merge_at_set_of(&style_expr_empty!("background.color: {color.clone()}"));
        outer_style.merge_at_set_of(&border_style);
        let btn = ui! {
            <Ui context={ctx}>
                <Div style={outer_style} catch_inputs={true}>
                    <Div style={style}/>
                </Div>
            </Ui>
        };
        btn
    }

    fn create_empty_slot(ctx: UiContext) -> Element {
        let mut style = uistyles::SLOT_OUTER_STYLE.clone();
        style.merge_at_set_of(&style_expr_empty!("background.color: @R.color/inv_slot_bg"));
        let btn = ui! {
            <Ui context={ctx}>
                <Div style={style}/>
            </Ui>
        };
        btn
    }

    fn create_slot_row(
        iter: &mut Iter<IngredientStack>,
        ctx: UiContext,
        slots_across: usize,
        buttons: &mut Vec<Element>,
    ) -> Option<Element> {
        let mut div_style = uistyles::CLEAR.clone();
        div_style.merge_at_set_of(&style_expr_empty!(
            "direction: horizontal; padding: none; margin: none;"
        ));

        let mut div = ui! {
            <Ui context={ctx.clone()}>
                <Div style={div_style}/>
            </Ui>>
        };

        let e = div.get_mut();

        for i in 0..slots_across {
            if let Some(next) = iter.next() {
                let slot = Self::create_slot(next, ctx.clone());
                buttons.push(slot.clone());
                e.add_child(slot.to_child());
            } else {
                if i == 0 {
                    return None;
                }
                let slot = Self::create_empty_slot(ctx.clone());
                e.add_child(slot.to_child());
            }
        }

        Some(div)
    }

    pub fn item_display_container(&self) -> &Element {
        &self.item_display_container
    }
}

unsafe impl Send for InventoryDisplay {}
unsafe impl Sync for InventoryDisplay {}

pub struct CurrentInvDisplay {
    root: Element,
    elem: Option<InventoryDisplay>,
}

impl CurrentInvDisplay {
    pub fn new(window: &mut Window) -> Self {
        let t = Self {
            root: ui! {
                <Ui context={window.ui().context()}>
                    <Div style={uistyles::OUTER_FRAME.clone()}/>
                </Ui>>
            },
            elem: None,
        };
        window.ui_mut().add_root(t.root.clone());
        t
    }

    pub fn open(&mut self, to: InventoryDisplay, window: &mut Window) {
        let ui = window.ui_mut();
        if let Some(this) = self.elem.take() {
            if let Some(_) = this.element {
                let this = self.root.get_mut();
                this.remove_all_children();
            }
        }
        if let Some(elem) = to.element.clone() {
            let this = self.root.get_mut();
            this.add_child(elem.to_child());
            this.add_child(to.item_display_container.clone().to_child());
        }
        self.elem = Some(to);
    }

    pub fn check_events(&mut self, window: &mut Window, game: &Game) {
        if let Some(inv) = &mut self.elem {
            inv.check_events(window, game);
            if let Some(player_inv) = &mut inv.player_inv {
                player_inv.check_events(window, game);
            }
        }
    }
}

unsafe impl Send for CurrentInvDisplay {}
unsafe impl Sync for CurrentInvDisplay {}
