use mvengine::ui::utils::ToRope;
use mvutils::state::State;
use mvutils::Savable;
use ropey::Rope;

pub const PERSISTENT_FILE: &str = "persistent.sav";

#[derive(Clone, Savable)]
pub struct PersistentGameData {
    pub last_ip: String,
}

impl PersistentGameData {
    pub fn new() -> Self {
        Self {
            last_ip: String::new(),
        }
    }

    pub fn to_loaded(self) -> PersistentLoadedData {
        PersistentLoadedData {
            last_ip: State::new(self.last_ip.to_rope()),
        }
    }

    pub fn from_loaded(loaded: &PersistentLoadedData) -> Self {
        Self {
            last_ip: loaded.last_ip.read().to_string(),
        }
    }
}

pub struct PersistentLoadedData {
    pub last_ip: State<Rope>,
}
