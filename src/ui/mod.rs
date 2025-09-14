pub mod display;
pub mod escape_screen;
pub mod mainscreen;
pub mod manager;
pub mod settings;
pub mod status_screen;

use crate::gameloop::GameHandler;
use log::{debug, error};
use mvengine::rendering::RenderContext;
use mvengine::ui::elements::{Element, UiElementCallbacks, UiElementStub};
use mvengine::ui::page::Page;
use mvengine::window::Window;
use mvutils::thread::ThreadSafe;
use std::any::Any;

pub struct GameUi {
    pub callbacks: Box<dyn GameUiCallbacks>,
    element: ThreadSafe<Element>,
}

impl GameUi {
    pub fn new(callbacks: impl GameUiCallbacks + 'static) -> Option<Self> {
        let rc = callbacks.get_elem();
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
        let name = self.callbacks.get_name();
        window.ui_mut().page_manager_mut().open(name);
    }

    pub fn check_events(&mut self, window: &mut Window, game_handler: &mut GameHandler) {
        self.callbacks.check_ui_events(window, game_handler);
    }
}

pub trait GameUiCallbacks: Page + Send + Sync {
    fn get_name(&self) -> &str;
    fn check_ui_events(&mut self, window: &mut Window, game_handler: &mut GameHandler);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
