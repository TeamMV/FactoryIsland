use crate::game::Game;
use log::{error, info};
use mvengine::input::registry::{Direction, RawInput};
use mvengine::input::Input;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use mvengine::input::consts::{Key, MouseButton};

pub const MOVE_FORWARD: &str = "move_forward";
pub const MOVE_BACK: &str = "move_back";
pub const MOVE_LEFT: &str = "move_left";
pub const MOVE_RIGHT: &str = "move_right";
pub const ESCAPE: &str = "escape";
pub const CHAT: &str = "chat";
pub const RELOAD_CHUNKS: &str = "reload_chunks";
pub const ROTATE_L: &str = "rotatel";
pub const ROTATE_R: &str = "rotater";

pub const PATH: &str = ".factoryisland/";

pub struct InputManager;

impl InputManager {
    pub fn init(game: &Game, input: &mut Input) {
        let actions = input.action_registry_mut();
        actions.create_action(MOVE_FORWARD);
        actions.create_action(MOVE_BACK);
        actions.create_action(MOVE_LEFT);
        actions.create_action(MOVE_RIGHT);
        actions.create_action(ESCAPE);
        actions.create_action(CHAT);
        actions.create_action(RELOAD_CHUNKS);
        actions.create_action(ROTATE_L);
        actions.create_action(ROTATE_R);

        //defaults, get overridden by file
        actions.bind_action(MOVE_FORWARD, vec![RawInput::KeyPress(Key::W)]);
        actions.bind_action(MOVE_BACK, vec![RawInput::KeyPress(Key::S)]);
        actions.bind_action(MOVE_LEFT, vec![RawInput::KeyPress(Key::A)]);
        actions.bind_action(MOVE_RIGHT, vec![RawInput::KeyPress(Key::D)]);
        actions.bind_action(ESCAPE, vec![RawInput::KeyPress(Key::Escape)]);
        actions.bind_action(CHAT, vec![RawInput::KeyPress(Key::Return)]);
        actions.bind_action(RELOAD_CHUNKS, vec![RawInput::KeyPress(Key::F3), RawInput::KeyPress(Key::R)]);
        actions.bind_action(ROTATE_L, vec![RawInput::Scroll(Direction::Up)]);
        actions.bind_action(ROTATE_R, vec![RawInput::Scroll(Direction::Down)]);

        let dir = game.configuration_directory();
        if let Ok(_) = input.load_actions(dir.path()) {
            info!("Loaded input actions from file");
        }
    }

    pub fn close(game: &Game, input: &mut Input) {
        let dir = game.configuration_directory();
        if let Err(e) = input.save_actions(dir.path()) {
            error!("Error when saving actions: {}", e);
        }
    }
}