use mvengine::ui::elements::UiElementStub;
use crate::ui::{mainscreen, GameUi};
use mvengine::window::Window;
use crate::gameloop::GameHandler;
use crate::ui::escape_screen::EscapeScreen;
use crate::ui::mainscreen::Mainscreen;

pub const AMT_UIS: usize = 2;

pub const UI_MAIN_SCREEN: usize = 0;
pub const UI_ESCAPE_SCREEN: usize = 1;

pub struct GameUiManager {
    current: Option<usize>,
    uis: [GameUi; AMT_UIS]
}

impl GameUiManager {
    pub fn create_all(window: &Window) -> Self {
        Self {
            current: None,
            uis: [
                GameUi::new(Mainscreen::new(window)).expect("vanilla stuff that cannot break"),
                GameUi::new(EscapeScreen::new(window)).expect("vanilla stuff that cannot break"),
            ]
        }
    }
    
    pub fn goto(&mut self, ui: usize, window: &mut Window) {
        if let Some(current) = self.current {
            if let Some(open) = self.uis.get_mut(current) {
                open.close(window);
            }
        }
        self.current = Some(ui);
        if let Some(elem) = self.uis.get_mut(ui) {
            elem.open(window);
        }
    }
    
    pub fn close_all(&mut self, window: &mut Window) {
        if let Some(current) = self.current {
            if let Some(open) = self.uis.get_mut(current) {
                open.close(window);
            }
        }
        self.current = None;
    }
    
    pub fn check_events(&mut self, window: &mut Window, game_handler: &mut GameHandler) {
        if let Some(current) = self.current {
            if let Some(ui) = self.uis.get_mut(current) {
                ui.check_events(window, game_handler);
            }
        }
    }
}