use mvengine::ui::context::UiResources;
use crate::res::R;
use mvengine_proc::resolve_resource;
use crate::gameloop::GameHandler;
use crate::ui::manager::UI_MAIN_SCREEN;
use crate::ui::GameUiCallbacks;
use mvengine::color::RgbColor;
use mvengine::input::consts::MouseButton;
use mvengine::net::DisconnectReason;
use mvengine::ui::elements::button::Button;
use mvengine::ui::elements::div::Div;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::elements::Element;
use mvengine::ui::elements::UiElementStub;
use mvengine::window::Window;
use mvengine::{expect_element_by_id, modify_style};
use mvengine::ui::styles::{UiStyle, UiValue};
use mvengine::ui::styles::enums::{ChildAlign, Position};
use mvengine::ui::styles::groups::SideStyle;
use mvengine::ui::styles::unit::Unit;
use mvengine_proc::{style_expr, ui};
use mvutils::thread::ThreadSafe;

pub struct EscapeScreen {
    elem: ThreadSafe<Element>,
    quit_btn: ThreadSafe<Element>,
    back_btn: ThreadSafe<Element>
}

impl EscapeScreen {
    pub fn new(window: &Window) -> Self {
        let main_style = style_expr!("position: absolute; width: 100%; height: 100%; child_align_x: middle; child_align_y: middle; background.resource: texture; background.texture: @R.drawable/bg; margin: none; padding: none;");

        let vert_style = UiStyle::stack_vertical();

        let widget_style = style_expr!("width: 5cm; height: 1cm;");

        let mut hover_style = widget_style.clone();
        modify_style!(hover_style.background.color = UiValue::Just(RgbColor::red()));


        let elem = ui! {
            <Ui context={window.ui().context()}>
                <Div id="escape_screen" style={main_style}>
                    <Div style={vert_style}>
                        <Button style={widget_style.clone()} id="quit">Quit</Button>
                        <Button style={widget_style} id="back">Back</Button>
                    </Div>
                </Div>
            </Ui>
        };
        
        let quit_btn = expect_element_by_id!(elem, "quit");
        let back_btn = expect_element_by_id!(elem, "back");
        
        Self {
            elem: ThreadSafe::new(elem),
            quit_btn: ThreadSafe::new(quit_btn),
            back_btn: ThreadSafe::new(back_btn),
        }
    }
}

impl GameUiCallbacks for EscapeScreen {
    fn element(&self) -> Element {
        self.elem.as_ref().clone()
    }

    fn check_ui_events(&mut self, window: &mut Window, game_handler: &mut GameHandler) {
        if let Some(event) = &self.back_btn.as_ref().get().state().events.click_event {
            if let MouseButton::Left = event.button {
                if let UiClickAction::Click = event.base.action {
                    println!("click back to game");
                    game_handler.ui_manager.close_all(window);
                }
            }
        }

        if let Some(event) = &self.quit_btn.as_ref().get().state().events.click_event {
            if let MouseButton::Left = event.button {
                if let UiClickAction::Click = event.base.action {
                    if let Some(client) = &mut game_handler.client { 
                        client.disconnect(DisconnectReason::Disconnected);
                    }
                    game_handler.client = None;
                    println!("click disconnect");
                    game_handler.ui_manager.goto(UI_MAIN_SCREEN ,window);
                }
            }
        }
    }
}