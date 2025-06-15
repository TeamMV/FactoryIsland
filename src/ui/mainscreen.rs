use mvengine::ui::context::UiResources;
use crate::gameloop::GameHandler;
use crate::res::R;
use crate::ui::GameUiCallbacks;
use mvengine::color::parse::parse_color;
use mvengine::graphics::Drawable;
use mvengine::input::consts::MouseButton;
use mvengine::net::client::Client;
use mvengine::ui::attributes::UiState;
use mvengine::ui::elements::button::Button;
use mvengine::ui::elements::div::Div;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::elements::textbox::TextBox;
use mvengine::ui::elements::UiElementStub;
use mvengine::ui::elements::{Element, UiElement};
use mvengine::ui::styles::enums::{BackgroundRes, ChildAlign, Position};
use mvengine::ui::styles::groups::SideStyle;
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

pub struct Mainscreen {
    elem: ThreadSafe<Element>,
    connect_btn: ThreadSafe<Element>,
    server_ip: UiState
}

impl Mainscreen {
    pub fn new(window: &Window) -> Self {
        let main_style = style_expr!("position: absolute; width: 100%; height: 100%; child_align_x: middle; child_align_y: middle; background.resource: texture; background.texture: @R.drawable/bg; margin: none; padding: none;");

        let vert_style = UiStyle::stack_vertical();
        
        let widget_style = style_expr!("width: 5cm; height: 1cm;");

        let mut hover_style = widget_style.clone();
        modify_style!(hover_style.background.color = UiValue::Just(parse_color("#3f48cc").unwrap()));

        let elem = ui! {
            <Ui context={window.ui().context()}>
                <Div id="mainscreen" style={main_style}>
                    <Div style={vert_style}>
                        <TextBox style={widget_style.clone()} id="ip_input" placeholder="SeverIP" content="127.0.0.1:4040"/>
                        <Button style={widget_style} id="connect">Connect</Button>
                    </Div>
                </Div>
            </Ui>
        };

        let connect_btn = expect_element_by_id!(elem, "connect");

        expect_inner_element_by_id_mut!(elem, Button, "connect", btn => {
            btn.body_mut().set_fade_time(200);
            btn.body_mut().set_hover_style(Some(hover_style.clone()));
        });
        
        let mut content = State::new(String::new()).map_identity();
        
        expect_inner_element_by_id_mut!(elem, TextBox, "ip_input", textbox => {
            textbox.body_mut().set_fade_time(200);
            textbox.body_mut().set_hover_style(Some(hover_style));
            content = textbox.content();
        });

        Self {
            elem: ThreadSafe::new(elem),
            connect_btn: ThreadSafe::new(connect_btn),
            server_ip: content,
        }
    }
}

impl GameUiCallbacks for Mainscreen {
    fn element(&self) -> Element {
        self.elem.as_ref().clone()
    }

    fn check_ui_events(&mut self, window: &mut Window, game_handler: &mut GameHandler) {
        if let Some(event) = &self.connect_btn.as_ref().get().state().events.click_event {
            if let MouseButton::Left = event.button {
                if let UiClickAction::Click = event.base.action {
                    let ip = self.server_ip.read().to_string();
                    let client = Client::connect(ip, game_handler.this.deref().clone()).expect("Cannot connect to local server");
                    game_handler.client = Some(client);

                    game_handler.ui_manager.close_all(window);
                }
            }
        }
    }
}