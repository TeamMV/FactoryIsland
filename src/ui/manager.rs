use crate::gameloop::GameHandler;
use crate::ui::escape_screen::EscapeScreen;
use crate::ui::mainscreen::Mainscreen;
use crate::ui::GameUi;
use mvengine::ui::elements::UiElementStub;
use mvengine::window::Window;
use crate::ui::settings::SettingsScreen;

pub const AMT_UIS: usize = 3;

pub const UI_MAIN_SCREEN: usize = 0;
pub const UI_ESCAPE_SCREEN: usize = 1;
pub const UI_SETTINGS_SCREEN: usize = 2;

pub struct GameUiManager {
    current: Option<usize>,
    pub uis: [GameUi; AMT_UIS]
}

impl GameUiManager {
    pub fn create_all(window: &mut Window) -> Self {
        let this = Self {
            current: None,
            uis: [
                GameUi::new(Mainscreen::new(window)).expect("vanilla stuff that cannot break"),
                GameUi::new(EscapeScreen::new(window)).expect("vanilla stuff that cannot break"),
                GameUi::new(SettingsScreen::new(window)).expect("vanilla stuff that cannot break"),
            ]
        };

        for gui in &this.uis {
            let e = gui.callbacks.get_elem();
            window.ui_mut().page_manager_mut().add_page(e);
        }

        this
    }
    
    pub fn goto(&mut self, ui: usize, window: &mut Window) {
        if let Some(elem) = self.uis.get_mut(ui) {
            elem.open(window);
            self.current = Some(ui);
        }
    }
    
    pub fn close_all(&mut self, window: &mut Window) {
        window.ui_mut().page_manager_mut().close_all();
        self.current = None;
    }

    pub fn go_back(&mut self, window: &mut Window) {
        let id = window.ui_mut().page_manager_mut().go_back();
        if let Some(id) = id.as_deref() {
            if let Some((meant, _)) = self.uis.iter().enumerate().find(|(_, g)| g.callbacks.get_name() == id) {
                self.current = Some(meant);
            }
        }
    }
    
    pub fn check_events(&mut self, window: &mut Window, game_handler: &mut GameHandler) {
        if let Some(current) = self.current {
            if let Some(ui) = self.uis.get_mut(current) {
                ui.check_events(window, game_handler);
            }
        }
    }
}