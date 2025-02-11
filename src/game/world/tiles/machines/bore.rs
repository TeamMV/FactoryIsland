use mvutils::Savable;

#[derive(Clone, Savable)]
pub struct BoreMachine {
    speed: f64
}

impl BoreMachine {
    pub fn new() -> Self {
        Self {
            speed: 10.0,
        }
    }
}