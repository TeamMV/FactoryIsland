use crate::world::tile_tex_mapper::get_tile_drawable;
use api::server::packets::common::TileKind;
use api::world::tiles::ObjectSource;
use mvengine::color::RgbColor;
use mvengine::graphics::comp::Drawable;
use mvengine::{enum_val_ref_mut, modify_style};
use mvengine::input::consts::MouseButton;
use mvengine::ui::elements::child::Child;
use mvengine::ui::elements::div::Div;
use mvengine::ui::elements::{Element, UiElement};
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::elements::UiElementStub;
use mvengine::ui::styles::{BackgroundRes, Origin, Position, SideStyle, UiStyle, UiValue, Unit};
use mvengine::utils::fuckumaxfornotmakingshitpub::ThreadSafe;
use mvengine::window::Window;
use mvengine_proc::ui;

pub struct TileSelection {
    selected_index: Option<usize>,
    root: ThreadSafe<Element>,
    buttons: Vec<ThreadSafe<Element>>
}

impl TileSelection {
    pub fn new(window: &Window, available_tiles: impl Iterator<Item=TileKind>) -> Self {
        let mut outer_style = UiStyle::stack_vertical();
        modify_style!(outer_style.position = UiValue::Just(Position::Absolute));
        modify_style!(outer_style.origin = UiValue::Just(Origin::BottomRight));
        modify_style!(outer_style.x = UiValue::Just(window.info().width as i32));
        modify_style!(outer_style.y = UiValue::Just(0));
        modify_style!(outer_style.background.resource = UiValue::None);
        
        let mut tile_style = UiStyle::default();
        modify_style!(tile_style.width = UiValue::Measurement(Unit::CM(7.0)));
        modify_style!(tile_style.height = UiValue::Measurement(Unit::CM(7.0)));
        modify_style!(tile_style.border.color = UiValue::Just(RgbColor::white()));
        modify_style!(tile_style.background.resource = UiValue::Just(BackgroundRes::Texture.into()));
        tile_style.padding = SideStyle::all_i32(0);
        tile_style.margin = SideStyle::block(UiValue::Measurement(Unit::CM(2.0)).to_resolve());
        
        let outer = ui! {
            <Ui context={window.ui().context()}>
                <Div id="tile_select_outer" style={outer_style}/>
            </Ui>
        };
        
        let div = outer.get_mut();
        
        let mut buttons = vec![];
        for (i, tile) in available_tiles.enumerate() {
            let mut style = tile_style.clone();
            modify_style!(style.background.texture = UiValue::Just(get_drawable(&tile).into()));
            modify_style!(style.transform.origin = UiValue::Just(Origin::Center));
            
            let mut hover = style.clone();
            modify_style!(hover.transform.scale! = UiValue::Just(1.2));
            
            let button = ui! {
                <Ui context={window.ui().context()}>
                    <Div class="tile_select" style={style}/>
                </Ui>
            };
            
            let elem = button.get_mut();
            let inner = enum_val_ref_mut!(UiElement, elem, Div);
            inner.body_mut().set_fade_time(200);
            inner.body_mut().set_hover_style(Some(hover));
            
            buttons.push(ThreadSafe::new(button.clone()));
            div.add_child(Child::Element(button))
        }
        
        Self {
            selected_index: None,
            root: ThreadSafe::new(outer),
            buttons,
        }
    }
    
    pub fn open(&self, window: &mut Window) {
        window.ui_mut().add_root(self.root.as_ref().clone());
    }
    
    pub fn close(&self, window: &mut Window) {
        window.ui_mut().remove_root(self.root.as_ref().clone());
    }
    
    pub fn check_events(&mut self) {
        for (i, button) in self.buttons.iter().enumerate() {
            let elem = button.as_ref().get_mut();
            if let Some(event) = &elem.state().events.click_event {
                if let UiClickAction::Click = event.base.action { 
                    if let MouseButton::Left = event.button {
                        if let Some(prev) = self.selected_index {
                            let prev_btn = self.buttons[prev].as_ref();
                            let b = prev_btn.get_mut();
                            let b = enum_val_ref_mut!(UiElement, b, Div);
                            let hover = b.body_mut().hover_style_mut();
                            if let Some(hover) = hover {
                                modify_style!(hover.border.color = UiValue::Auto);
                            }
                            let init = b.body_mut().initial_style_mut();
                            if let Some(init) = init {
                                modify_style!(init.border.color = UiValue::Auto);
                            }
                            let style = b.style_mut();
                            modify_style!(style.border.color = UiValue::Auto);
                        }
                        self.selected_index = Some(i);
                        let b = enum_val_ref_mut!(UiElement, elem, Div);
                        let hover = b.body_mut().hover_style_mut();
                        if let Some(hover) = hover {
                            modify_style!(hover.border.color = UiValue::Just(RgbColor::yellow()));
                        }
                        let init = b.body_mut().initial_style_mut();
                        if let Some(init) = init {
                            modify_style!(init.border.color = UiValue::Just(RgbColor::yellow()));
                        }
                        let style = b.style_mut();
                        modify_style!(style.border.color = UiValue::Just(RgbColor::yellow()));
                    }
                }
            }
        }
    }
}

fn get_drawable(kind: &TileKind) -> Drawable {
    if let ObjectSource::Mod(m, mapper) = &kind.source {
        mapper.map(0)
    } else {
        get_tile_drawable(kind.id, 0)
    }
}