use mvutils::save::Savable;
use mvutils::state::State;
use mvutils::Savable;

pub(crate) const SETTINGS_FILE: &str = "settings.sav";

#[derive(Clone, Savable)]
pub struct GameSettings {
    pub ssao_shader: State<bool>,
    pub cloud_shader: State<bool>,
}

impl GameSettings {
    pub fn new() -> Self {
        Self {
            ssao_shader: State::new(true),
            cloud_shader: State::new(true),
        }
    }
}
