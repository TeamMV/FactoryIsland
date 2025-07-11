use crate::gameloop::GameHandler;
use crate::ui::manager::{UI_MAIN_SCREEN, UI_SETTINGS_SCREEN};
use crate::ui::GameUiCallbacks;
use crate::uistyles;
use mvengine::color::RgbColor;
use mvengine::input::consts::MouseButton;
use mvengine::net::DisconnectReason;
use mvengine::ui::elements::button::Button;
use mvengine::ui::elements::div::Div;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::elements::Element;
use mvengine::ui::elements::UiElementStub;
use mvengine::ui::page::Page;
use mvengine::ui::styles::{UiStyle, UiValue};
use mvengine::window::Window;
use mvengine::{expect_element_by_id, modify_style};
use mvengine_proc::ui;
use mvutils::thread::ThreadSafe;
use std::any::Any;

pub struct EscapeScreen {
    elem: ThreadSafe<Element>,
    quit_btn: ThreadSafe<Element>,
    back_btn: ThreadSafe<Element>,
    settings_btn: ThreadSafe<Element>,
}

impl EscapeScreen {
    pub fn new(window: &Window) -> Self {
        let mut main_style = uistyles::BG.clone();
        main_style.merge_at_set_of(&uistyles::CLEAR);

        let mut vert_style = UiStyle::stack_vertical();
        vert_style.merge_at_set_of(&uistyles::CLEAR);

        let widget_style = uistyles::PRESET.clone();

        let mut hover_style = widget_style.clone();
        modify_style!(hover_style.background.color = UiValue::Just(RgbColor::red()));


        let elem = ui! {
            <Ui context={window.ui().context()}>
                <Div id="escape" style={main_style}>
                    <Div style={vert_style}>
                        <Button style={widget_style.clone()} id="back">Back</Button>
                        <Button style={widget_style.clone()} id="quit">Quit</Button>
                        <Button style={widget_style.clone()} id="settings">Settings</Button>
                    </Div>
                </Div>
            </Ui>
        };
        
        let quit_btn = expect_element_by_id!(elem, "quit");
        let back_btn = expect_element_by_id!(elem, "back");
        let settings_btn = expect_element_by_id!(elem, "settings");

        Self {
            elem: ThreadSafe::new(elem),
            quit_btn: ThreadSafe::new(quit_btn),
            back_btn: ThreadSafe::new(back_btn),
            settings_btn: ThreadSafe::new(settings_btn),
        }
    }
}

impl Page for EscapeScreen {
    fn get_elem(&self) -> Element {
        self.elem.as_ref().clone()
    }
}

impl GameUiCallbacks for EscapeScreen {
    fn get_name(&self) -> &str {
        "escape"
    }

    fn check_ui_events(&mut self, window: &mut Window, game_handler: &mut GameHandler) {
        if let Some(event) = &self.back_btn.as_ref().get().state().events.click_event {
            if let MouseButton::Left = event.button {
                if let UiClickAction::Click = event.base.action {
                    game_handler.ui_manager.close_all(window);
                }
            }
        }

        if let Some(event) = &self.settings_btn.as_ref().get().state().events.click_event {
            if let MouseButton::Left = event.button {
                if let UiClickAction::Click = event.base.action {
                    game_handler.ui_manager.goto(UI_SETTINGS_SCREEN, window);
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
                    game_handler.ui_manager.goto(UI_MAIN_SCREEN ,window);
                }
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}