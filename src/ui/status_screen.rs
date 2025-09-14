use crate::gameloop::GameHandler;
use crate::ui::GameUiCallbacks;
use crate::uistyles;
use mvengine::expect_element_by_id;
use mvengine::input::consts::MouseButton;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::elements::prelude::*;
use mvengine::ui::elements::Element;
use mvengine::ui::page::Page;
use mvengine::ui::styles::UiStyle;
use mvengine::window::Window;
use mvengine_proc::ui;
use mvutils::lazy;
use mvutils::state::State;
use mvutils::thread::ThreadSafe;
use ropey::Rope;
use std::any::Any;

lazy! {
    pub static STATUS_MSG: State<Rope> = State::new(Rope::new());
}

pub struct StatusScreen {
    elem: ThreadSafe<Element>,
    back_btn: ThreadSafe<Element>,
}

impl StatusScreen {
    pub fn new(window: &Window) -> Self {
        let main_style = uistyles::BG.clone();
        let mut vert_style = UiStyle::stack_vertical();
        vert_style.merge_at_set_of(&uistyles::CLEAR);

        let widget_style = uistyles::PRESET.clone();
        let text_style = uistyles::CLEAR_PRESET.clone();

        let state = STATUS_MSG.clone();

        let elem = ui! {
            <Ui context={window.ui().context()}>
                <Div id="status_screen" style={main_style}>
                    <Div style={vert_style}>
                        <Button style={text_style} id="message">{state.map_identity()}</Button>
                        <Button style={widget_style} id="back">Back</Button>
                    </Div>
                </Div>
            </Ui>
        };

        let back_btn = expect_element_by_id!(elem, "back");

        Self {
            elem: ThreadSafe::new(elem),
            back_btn: ThreadSafe::new(back_btn),
        }
    }
}

impl Page for StatusScreen {
    fn get_elem(&self) -> Element {
        self.elem.as_ref().clone()
    }
}

impl GameUiCallbacks for StatusScreen {
    fn get_name(&self) -> &str {
        "status_screen"
    }

    fn check_ui_events(&mut self, window: &mut Window, game_handler: &mut GameHandler) {
        if self.back_btn.was_left_clicked() {
            game_handler.ui_manager.go_back(window);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
