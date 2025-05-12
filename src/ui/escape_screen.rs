use std::ops::Deref;
use mvengine::color::RgbColor;
use mvengine::{expect_element_by_id, modify_style};
use mvengine::input::consts::MouseButton;
use mvengine::net::client::Client;
use mvengine::net::DisconnectReason;
use mvengine::ui::elements::Element;
use mvengine::ui::styles::{ChildAlign, Position, SideStyle, UiStyle, UiValue, Unit};
use mvengine::utils::fuckumaxfornotmakingshitpub::ThreadSafe;
use mvengine::window::Window;
use mvengine_proc::ui;
use crate::gameloop::GameHandler;
use crate::ui::GameUiCallbacks;
use mvengine::ui::elements::button::Button;
use mvengine::ui::elements::div::Div;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::elements::textbox::TextBox;
use mvengine::ui::elements::UiElementStub;
use crate::ui::manager::UI_MAIN_SCREEN;

pub struct EscapeScreen {
    elem: ThreadSafe<Element>,
    quit_btn: ThreadSafe<Element>,
    back_btn: ThreadSafe<Element>
}

impl EscapeScreen {
    pub fn new(window: &Window) -> Self {
        let mut main_style = UiStyle::default();
        modify_style!(main_style.position = UiValue::Just(Position::Absolute));
        modify_style!(main_style.width = UiValue::Just(window.info().width as i32));
        modify_style!(main_style.height = UiValue::Just(window.info().height as i32));
        modify_style!(main_style.child_align_x = UiValue::Just(ChildAlign::Middle));
        modify_style!(main_style.child_align_y = UiValue::Just(ChildAlign::Middle));
        main_style.padding = SideStyle::all_i32(0);
        main_style.margin = SideStyle::all_i32(0);

        let vert_style = UiStyle::stack_vertical();

        let mut widget_style = UiStyle::default();
        modify_style!(widget_style.width = UiValue::Measurement(Unit::BeardFortnight(50.0)));
        modify_style!(widget_style.height = UiValue::Measurement(Unit::CM(5.0)));

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
                    game_handler.ui_manager.close_all(window);
                }
            }
        }

        if let Some(event) = &self.quit_btn.as_ref().get().state().events.click_event {
            if let MouseButton::Left = event.button {
                if let UiClickAction::Click = event.base.action {
                    game_handler.ui_manager.goto(UI_MAIN_SCREEN ,window);
                    if let Some(client) = &mut game_handler.client { 
                        client.disconnect(DisconnectReason::Disconnected);
                    }
                    game_handler.client = None;
                }
            }
        }
    }
}