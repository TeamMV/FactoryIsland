use crate::game::Game;
use crate::gameloop::GameHandler;
use crate::ui::GameUiCallbacks;
use crate::uistyles;
use mvengine::expect_element_by_id;
use mvengine::input::consts::MouseButton;
use mvengine::modify_style;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::elements::prelude::*;
use mvengine::ui::elements::Element;
use mvengine::ui::page::Page;
use mvengine::window::Window;
use mvengine_proc::{style_expr_empty, ui};
use mvutils::state::State;
use mvutils::thread::ThreadSafe;
use std::any::Any;

pub struct SettingsScreen {
    elem: ThreadSafe<Element>,
    back_btn: ThreadSafe<Element>,

    pub enable_clouds: State<bool>,
    pub enable_ssao: State<bool>,
    pub indicator_circle: State<bool>
}

impl SettingsScreen {
    pub fn new(window: &Window, game: &Game) -> Self {
        let main_style = uistyles::BG.clone();
        let widget = uistyles::PRESET.clone();
        let checkbox_style = uistyles::CHECKBOX_PRESET.clone();
        let slider_style = uistyles::SLIDER_PRESET.clone();
        let mut clear_style = uistyles::CLEAR.clone();
        clear_style.merge_at_set_of(&style_expr_empty!("width: auto; height: auto; direction: horizontal;"));

        let enable_clouds = game.settings.cloud_shader.clone();
        let enable_ssao = game.settings.ssao_shader.clone();
        let indicator_circle = game.settings.indicator_circle.clone();

        let elem = ui! {
            <Ui context={window.ui().context()}>
                <Div style={main_style} id="settings">
                    <Div style={uistyles::OUTER_FRAME.clone()}>
                        <Div style={uistyles::FRAME.clone()}>
                            <Button style={uistyles::CLEAR_PRESET.clone()}>- Settings -</Button>
                            <CheckBox selected={enable_clouds.clone()} style={checkbox_style.clone()}>Cloud Shader</CheckBox>
                            <CheckBox selected={enable_ssao.clone()} style={checkbox_style.clone()}>SSAO Shader</CheckBox>
                            <CheckBox selected={indicator_circle.clone()} style={checkbox_style.clone()}>Fat indicator circle</CheckBox>
                            <Div style={clear_style}>
                                <Button style={uistyles::CLEAR_PRESET.clone()}>Simulation distance:</Button>
                                <Slider style={slider_style} range="1..10@1"/>
                            </Div>
                            <Button style={widget.clone()} id="back_btn">Back</Button>
                        </Div>            
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
            indicator_circle,
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