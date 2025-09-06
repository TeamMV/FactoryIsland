use std::ops::Deref;
use std::sync::Arc;
use mvengine::ui::elements::child::ToChildFromIterator;
use mvengine::ui::elements::text::Text;
use mvengine::ui::elements::child::ToChild;
use mvengine::ui::elements::button::Button;
use mvengine::ui::elements::div::Div;
use mvengine::ui::elements::textbox::TextBox;
use mvengine::ui::elements::Element;
use mvengine::ui::elements::UiElementStub;
use mvengine::ui::elements::UiElement;
use mvengine::window::Window;
use mvengine::{expect_element_by_id, expect_inner_element_by_id_mut, modify_style};
use mvengine::net::server::ClientEndpoint;
use mvengine::ui::context::UiContext;
use mvengine::utils::RopeFns;
use mvengine_proc::style_expr;
use mvengine_proc::ui;
use mvutils::enum_val_ref_mut;
use mvutils::state::State;
use mvutils::thread::ThreadSafe;
use ropey::Rope;
use api::server::packets::player::{OtherPlayerChatPacket, PlayerChatPacket};
use api::server::ServerBoundPacket;
use crate::gameloop::FactoryIslandClient;

pub struct Chat {
    pub open: bool,
    element: ThreadSafe<Element>,
    scroll_div: ThreadSafe<Element>,
    chat_state: State<Rope>,
    context: ThreadSafe<UiContext>,
}

impl Chat {
    pub fn new(window: &mut Window) -> Self {
        let chat_state = State::new(Rope::new());

        let element = ui! {
            <Ui context={window.ui().context()}>
                <Div id="chat_container" style="position: absolute; x: 0; y: 0; width: 100%; background.resource: none; border.resource: none; padding: 0.5cm; margin: none; direction: vertical;">
                    <TextBox id="chat_input" content={chat_state.clone()} style="margin: none; padding: none; width: 100%; height: 1cm; border.resource: none; background.color: #00000044; text.color: white; text.align_x: start; text.size: 100%;"/>
                </Div>
            </Ui>
        };
        
        let scroll_elem = ui! {
            <Ui context={window.ui().context()}>
                <Div id="chat_scroll_div" style="position: absolute; x: 0.5cm; y: 3cm; width: 15cm; margin: none; padding: none; background.color: #00000044; border.resource: none; direction: vertical">

                </Div>
            </Ui>
        };
        
        window.ui_mut().add_root(scroll_elem.clone());

        Self {
            open: false,
            element: ThreadSafe::new(element),
            scroll_div: ThreadSafe::new(scroll_elem),
            chat_state,
            context: ThreadSafe::new(window.ui().context()),
        }
    }

    pub fn open(&self, window: &mut Window) {
        let elem = self.element.as_ref().clone();
        expect_inner_element_by_id_mut!(elem, TextBox, "chat_input", input => {
            input.focus_now();
        });

        window.ui_mut().add_root(elem);
    }

    pub fn close(&self, window: &mut Window) {
        window.ui_mut().remove_root(self.element.as_ref().clone());
    }

    pub fn toggle(&mut self, window: &mut Window, client: &mut FactoryIslandClient) {
        self.open = !self.open;
        if self.open {
            self.open(window);
            self.chat_state.write().clear();
        } else {
            self.close(window);

            let message = self.chat_state.read();
            let message = message.to_string();
            
            client.send(ServerBoundPacket::PlayerChat(PlayerChatPacket {
                message,
            }));
        }
    }

    pub fn push_message(&mut self, packet: OtherPlayerChatPacket) {
        let name = format!("<{}> ", packet.player.data.profile.name);
        self.create_message_element(name, packet.message);
    }
    
    fn create_message_element(&self, name: String, message: String) {
        let name_button = ui! {
            <Ui context={self.context.as_ref().clone()}>
                <Button style="background.resource: none; border.resource: none; text.color: white; height: 1cm; margin: none; padding: none; text.align_y: start;">{name}</Button>
            </Ui>
        };
        
        let message_text = ui! {
            <Ui context={self.context.as_ref().clone()}>
                <Text style="background.resource: none; border.resource: none; text.color: white; width: 10cm; margin: none; margin.top: 3mm; margin.bottom: 3mm; padding: none;">{message}</Text>
            </Ui>
        };
        
        let div = ui! {
            <Ui context={self.context.as_ref().clone()}>
                <Div style="background.resource: none; border.resource: none; margin: none; padding: none;">{[name_button, message_text].into_iter()}</Div>
            </Ui>
        };
        
        self.scroll_div.get_mut().add_child(div.to_child());
    }
}