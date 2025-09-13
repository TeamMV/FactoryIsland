use std::any::Any;
use crate::gameloop::GameHandler;
use crate::res::R;
use crate::ui::GameUiCallbacks;
use mvengine::color::parse::parse_color;
use mvengine::input::consts::MouseButton;
use mvengine::net::client::Client;
use mvengine::ui::attributes::{ToRope, UiState};
use mvengine::ui::context::UiResources;
use mvengine::ui::elements::button::Button;
use mvengine::ui::elements::div::Div;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::elements::textbox::TextBox;
use mvengine::ui::elements::UiElementStub;
use mvengine::ui::elements::{Element, UiElement};
use mvengine::ui::styles::{UiStyle, UiValue};
use mvengine::window::Window;
use mvengine::{expect_element_by_id, expect_inner_element_by_id_mut, modify_style};
use mvengine_proc::resolve_resource;
use mvengine_proc::style_expr;
use mvengine_proc::ui;
use mvutils::enum_val_ref_mut;
use mvutils::state::State;
use mvutils::thread::ThreadSafe;
use std::ops::Deref;
use log::warn;
use mvengine::ui::page::Page;
use ropey::Rope;
use crate::game::Game;
use crate::ui::manager::{UI_SETTINGS_SCREEN, UI_STATUS_SCREEN};
use crate::ui::status_screen::STATUS_MSG;
use crate::uistyles;

pub struct Mainscreen {
    elem: ThreadSafe<Element>,
    connect_btn: ThreadSafe<Element>,
    settings_btn: ThreadSafe<Element>,
    server_ip: UiState,
    pub last_ip: State<Rope>,
}

impl Mainscreen {
    pub fn new(window: &Window, game: &Game) -> Self {
        let main_style = uistyles::BG.clone();

        let mut vert_style = UiStyle::stack_vertical();
        vert_style.merge_at_set_of(&uistyles::CLEAR);
        
        let widget_style = uistyles::PRESET.clone();

        let mut hover_style = widget_style.clone();
        modify_style!(hover_style.background.color = UiValue::Just(parse_color("#3f48cc").unwrap()));
        
        let last_ip = game.persistent_game_data.last_ip.clone();

        let elem = ui! {
            <Ui context={window.ui().context()}>
                <Div id="mainscreen" style={main_style}>
                    <Div style={vert_style}>
                        <TextBox style={uistyles::EDIT_PRESET.clone()} id="ip_input" placeholder="SeverIP" content={last_ip.clone()}/>
                        <Button style={widget_style.clone()} id="connect">Connect</Button>
                        <Button style={widget_style.clone()} id="settings">Settings</Button>
                    </Div>
                </Div>
            </Ui>
        };

        let connect_btn = expect_element_by_id!(elem, "connect");
        let settings_btn = expect_element_by_id!(elem, "settings");

        expect_inner_element_by_id_mut!(elem, Button, "connect", btn => {
            //btn.body_mut().set_fade_time(200);
            //btn.body_mut().set_hover_style(Some(hover_style.clone()));
        });
        
        let mut content = State::new(Rope::new()).map_identity();
        
        expect_inner_element_by_id_mut!(elem, TextBox, "ip_input", textbox => {
            //textbox.body_mut().set_fade_time(200);
            //textbox.body_mut().set_hover_style(Some(hover_style));
            content = textbox.content();
        });

        Self {
            elem: ThreadSafe::new(elem),
            connect_btn: ThreadSafe::new(connect_btn),
            settings_btn: ThreadSafe::new(settings_btn),
            server_ip: content,
            last_ip,
        }
    }
}

impl Page for Mainscreen {
    fn get_elem(&self) -> Element {
        self.elem.as_ref().clone()
    }
}

impl GameUiCallbacks for Mainscreen {
    fn get_name(&self) -> &str {
        "mainscreen"
    }

    fn check_ui_events(&mut self, window: &mut Window, game_handler: &mut GameHandler) {
        if let Some(event) = &self.connect_btn.as_ref().get().state().events.click_event {
            if let MouseButton::Left = event.button {
                if let UiClickAction::Click = event.base.action {
                    let ip = self.server_ip.read().to_string();
                    let conn = Client::connect(ip.clone(), game_handler.this.deref().upgrade().expect("This can never be invalid!"));
                    if let Some(client) = conn {
                        game_handler.client = Some(client);

                        game_handler.ui_manager.close_all(window);
                    } else {
                        warn!("Cannot connect to server: {ip}");
                        let mut lock = STATUS_MSG.write();
                        *lock = Rope::from_str("Cannot connect to server!");
                        game_handler.ui_manager.goto(UI_STATUS_SCREEN, window);
                    }
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
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}