use crate::gameloop::GameHandler;
use crate::ui::GameUiCallbacks;
use mvengine::color::RgbColor;
use mvengine::input::consts::MouseButton;
use mvengine::net::client::Client;
use mvengine::ui::elements::button::Button;
use mvengine::ui::elements::div::Div;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::elements::textbox::TextBox;
use mvengine::ui::elements::UiElementStub;
use mvengine::ui::elements::{Element, UiElement};
use mvengine::ui::styles::{ChildAlign, Position, SideStyle, UiStyle, UiValue, Unit};
use mvengine::utils::fuckumaxfornotmakingshitpub::ThreadSafe;
use mvengine::window::Window;
use mvengine::{enum_val_ref_mut, expect_element_by_id, expect_inner_element_by_id_mut, modify_style};
use mvengine_proc::ui;
use mvutils::state::State;
use std::ops::Deref;

pub struct Mainscreen {
    elem: ThreadSafe<Element>,
    connect_btn: ThreadSafe<Element>,
    server_ip: State<String>
}

impl Mainscreen {
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
                <Div id="mainscreen" style={main_style}>
                    <Div style={vert_style}>
                        <TextBox style={widget_style.clone()} id="ip_input" placeholder="SeverIP"/>
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
        
        let mut content = State::new(String::new());
        
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