use std::any::Any;
use mvengine::expect_element_by_id;
use mvengine::input::consts::MouseButton;
use crate::gameloop::GameHandler;
use crate::ui::GameUiCallbacks;
use crate::uistyles;
use mvengine::ui::elements::Element;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::page::Page;
use mvengine::window::Window;
use mvengine_proc::ui;
use mvutils::thread::ThreadSafe;
use mvengine::ui::elements::prelude::*;
use mvutils::state::State;
use crate::game::Game;

pub struct SettingsScreen {
    elem: ThreadSafe<Element>,
    back_btn: ThreadSafe<Element>,

    pub enable_clouds: State<bool>,
    pub enable_ssao: State<bool>,
}

impl SettingsScreen {
    pub fn new(window: &Window, game: &Game) -> Self {
        let main_style = uistyles::BG.clone();
        let widget = uistyles::PRESET.clone();

        let enable_clouds = game.settings.cloud_shader.clone();
        let enable_ssao = game.settings.ssao_shader.clone();

        let elem = ui! {
            <Ui context={window.ui().context()}>
                <Div style={main_style} id="settings">
                    <Div style={uistyles::FRAME.clone()}>
                        <Button style={uistyles::CLEAR_PRESET.clone()}>- Settings -</Button>
                        <CheckBox selected={enable_clouds.clone()} style={widget.clone()}>Cloud Shader</CheckBox>
                        <CheckBox selected={enable_ssao.clone()} style={widget.clone()}>SSAO Shader</CheckBox>
                        <Button style={widget.clone()} id="back_btn">Back</Button>
                    </Div>
                </Div>
            </Ui>
        };

        let back_btn = expect_element_by_id!(elem, "back_btn");

        Self {
            elem: ThreadSafe::new(elem),
            back_btn: ThreadSafe::new(back_btn),
            enable_clouds,
            enable_ssao,
        }
    }
}

impl Page for SettingsScreen {
    fn get_elem(&self) -> Element {
        self.elem.as_ref().clone()
    }
}

impl GameUiCallbacks for SettingsScreen {
    fn get_name(&self) -> &str {
        "settings"
    }

    fn check_ui_events(&mut self, window: &mut Window, game_handler: &mut GameHandler) {
        if let Some(event) = &self.back_btn.as_ref().get().state().events.click_event {
            if let MouseButton::Left = event.button {
                if let UiClickAction::Click = event.base.action {
                    game_handler.ui_manager.go_back(window);
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