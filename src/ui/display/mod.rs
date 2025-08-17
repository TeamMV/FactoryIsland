pub mod chat;

use mvengine::color::RgbColor;
use mvengine::ui::styles::{UiStyle, UiValue, EMPTY_STYLE};
use mvengine::ui::elements::button::Button;
use mvengine::ui::elements::child::{Child, ToChildFromIterator};
use mvengine::{expect_element_by_id, modify_style};
use mvengine_proc::style_expr;
use crate::world::tile_tex_mapper::get_tile_drawable;
use api::server::packets::common::TileKind;
use mvengine::graphics::Drawable;
use mvengine::input::consts::MouseButton;
use mvengine::ui::elements::child::ToChild;
use mvengine::ui::elements::div::Div;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::elements::UiElementStub;
use mvengine::ui::elements::Element;
use mvengine::ui::styles::enums::BackgroundRes;
use mvengine::window::Window;
use mvengine_proc::ui;
use mvutils::lazy;
use mvutils::thread::ThreadSafe;
use api::registry::ObjectSource;

lazy! {
    static SELECT_STYLE: UiStyle = {
        let mut empty = EMPTY_STYLE.clone();
        modify_style!(empty.border.color = UiValue::Just(RgbColor::yellow()));
        modify_style!(empty.border.resource = UiValue::Just(BackgroundRes::Color.into()));
        empty
    };

    static NO_SELECT_STYLE: UiStyle = {
        let mut empty = EMPTY_STYLE.clone();
       modify_style!(empty.border.color = UiValue::Just(RgbColor::black()));
        modify_style!(empty.border.resource = UiValue::None);
        empty
    };
}

pub struct TileSelection {
    selected_index: Option<usize>,
    tiles: Vec<TileKind>,
    root: ThreadSafe<Element>,
    buttons: Vec<ThreadSafe<Element>>
}

impl TileSelection {
    pub fn new(window: &Window, mut available_tiles: impl Iterator<Item=TileKind>) -> Self {
        let tiles = available_tiles.filter(|tile| tile.id != 0).collect::<Vec<_>>();
        let mut available_tiles = tiles.clone().into_iter();
        let outer = ui! {
            <Ui context={window.ui().context()}>
                <Div id="tile_selection" style="origin: bottom_right; position: absolute; x: 100%; y: 0; height: 10cm; direction: vertical; margin: none; background.resource: color; background.color: #00000044; border.resource: none;">
                    <Div id="button_container" style="direction: vertical; padding: none; margin: none; margin.right: 0.5cm; background.resource: none; border.resource: none;"/>
                </Div>
            </Ui>
        };

        let container = expect_element_by_id!(outer, "button_container");

        let mut buttons = vec![];
        while let Ok((bns, r)) = Self::create_row(window, &mut available_tiles) {
            container.get_mut().add_child(r.to_child());
            buttons.extend(bns);
        }

        let buttons = buttons.into_iter().map(|x| ThreadSafe::new(x)).collect();

        Self {
            selected_index: None,
            root: ThreadSafe::new(outer),
            buttons,
            tiles,
        }
    }

    fn create_row(window: &Window, available_tiles: &mut impl Iterator<Item=TileKind>) -> Result<(Vec<Element>, Element), ()> {
        let buttons = match available_tiles.next_chunk::<5>() {
            Ok(chunk) => {
                chunk.map(|tile| Self::create_button(window, tile)).to_vec()
            }
            Err(part_chunk) => {
                if part_chunk.is_empty() {
                    return Err(());
                } else {
                    part_chunk.map(|tile| Self::create_button(window, tile)).collect::<Vec<_>>()
                }
            }
        };

        let elem = ui! {
            <Ui context={window.ui().context()}>
                <Div style="background.resource: none; border.resource: none; margin: none; padding: none;">
                    {buttons.clone().into_iter()}
                </Div>
            </Ui>
        };

        Ok((buttons, elem))
    }

    fn create_button(window: &Window, tile_kind: TileKind) -> Element {
        let drawable = get_drawable(&tile_kind);

        let elem = ui! {
            <Ui context={window.ui().context()}>
                <Button style="width: 1.5cm; height: 1.5cm; background.resource: texture; background.texture: {drawable.clone()}; margin: 1bc;"/>
            </Ui>
        };
        elem
    }
    
    pub fn open(&self, window: &mut Window, parent: Element) {
        parent.get_mut().add_child(self.root.as_ref().clone().to_child());
    }
    
    pub fn close(&self, window: &mut Window) {
        if let Some(parent) = &self.root.as_ref().get().state().parent {
            let parent = parent.get_mut();
            parent.remove_child_by_id("tile_selection");
        }

        window.ui_mut().remove_root(self.root.as_ref().clone());
    }
    
    pub fn check_events(&mut self) {
        for (i, button) in self.buttons.iter().enumerate() {
            let elem = button.as_ref().get_mut();
            if let Some(event) = &elem.state().events.click_event {
                if let UiClickAction::Click = event.base.action {
                    if let MouseButton::Left = event.button {
                        if let Some(prev) = self.selected_index {
                            let prev_btn = &self.buttons[prev];
                            prev_btn.as_ref().get_mut().style_mut().merge_at_set_of(&NO_SELECT_STYLE);
                        }
                        self.selected_index = Some(i);
                        elem.style_mut().merge_at_set_of(&SELECT_STYLE);
                    }
                }
            }
        }
    }

    pub fn selected_tile(&self) -> Option<&TileKind> {
        if let Some(idx) = self.selected_index {
            Some(&self.tiles[idx])
        } else {
            None
        }
    }
}

fn get_drawable(kind: &TileKind) -> Drawable {
    if let ObjectSource::Mod(m) = &kind.source {
        Drawable::missing() //first get the rest to compile again
    } else {
        get_tile_drawable(kind.id, 0)
    }
}