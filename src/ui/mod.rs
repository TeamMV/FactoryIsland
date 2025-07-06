pub mod manager;
pub mod mainscreen;
pub mod escape_screen;
pub mod display;

use log::error;
use mvengine::rendering::RenderContext;
use mvengine::ui::elements::{Element, UiElementCallbacks, UiElementStub};
use mvengine::window::Window;
use mvutils::thread::ThreadSafe;
use crate::gameloop::GameHandler;

pub struct GameUi {
    callbacks: Box<dyn GameUiCallbacks>,
    element: ThreadSafe<Element>
}

impl GameUi {
    pub fn new(callbacks: impl GameUiCallbacks + 'static) -> Option<Self> {
        let rc = callbacks.element();
        let r = rc.get();
        if r.attributes().id.is_none() {
            error!("Cannot create GameUi since the provided ui lacks an id!");
            None
        } else {
            Some(Self {
                callbacks: Box::new(callbacks),
                element: ThreadSafe::new(rc),
            })
        }
    }
    
    pub fn open(&self, window: &mut Window) {
        window.ui_mut().add_root(self.element.as_ref().clone());
    }
    
    pub fn close(&self, window: &mut Window) {
        window.ui_mut().remove_root(self.element.as_ref().clone());
    }
    
    pub fn check_events(&mut self, window: &mut Window, game_handler: &mut GameHandler) {
        self.callbacks.check_ui_events(window, game_handler);
    }
}

pub trait GameUiCallbacks: Send + Sync {
    fn element(&self) -> Element;
    fn check_ui_events(&mut self, window: &mut Window, game_handler: &mut GameHandler);
}