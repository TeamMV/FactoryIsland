use std::any::Any;
use std::fs::File;
use std::io::{Read, Write};
use bytebuffer::{ByteBuffer, Endian};
use log::{debug, error, warn};
use mvutils::bytebuffer::ByteBufferExtras;
use mvutils::save::Savable;
use crate::game::Game;
use crate::ui::manager::{GameUiManager, UI_SETTINGS_SCREEN};
use crate::ui::settings::{GameSettingsFile, SettingsScreen};

const SETTINGS_FILE: &str = "settings.sav";

pub fn save_settings(game: &Game, ui_manager: &GameUiManager) {
    let screen = &ui_manager.uis[UI_SETTINGS_SCREEN];
    let mut file = game.configuration_directory().clone();
    file.push(SETTINGS_FILE);
    let saved = screen.callbacks.as_any();
    if let Some(screen) = saved.downcast_ref::<SettingsScreen>() {
        let saved = screen.create_save();
        if let Ok(mut file) = File::options().write(true).truncate(true).create(true).open(&file) {
            let mut buffer = ByteBuffer::new_le();
            saved.save(&mut buffer);
            let r = file.write_all(buffer.as_bytes());
            if r.is_err() {
                error!("Failed to save settings: {:?}", r.unwrap_err());
            } else {
                debug!("Saved settings!");
            }
        }
    } else {
        warn!("Failed to downcast the SettingsScreen!");
    }
}

pub fn load_settings(game: &Game, game_ui_manager: &mut GameUiManager) {
    let mut path = game.configuration_directory().clone();
    path.push(SETTINGS_FILE);

    if let Ok(mut file) = File::open(&path) {
        let mut data = Vec::new();
        if let Err(e) = file.read_to_end(&mut data) {
            error!("Failed to read settings file: {:?}", e);
            return;
        }

        let mut buffer = ByteBuffer::from_vec(data);
        buffer.set_endian(Endian::LittleEndian);
        match GameSettingsFile::load(&mut buffer) {
            Ok(settings_file) => {
                let screen = &mut game_ui_manager.uis[UI_SETTINGS_SCREEN];
                let screen_callbacks = &mut *screen.callbacks;

                let screen_any = screen_callbacks.as_any_mut();
                if let Some(settings_screen) = screen_any.downcast_mut::<SettingsScreen>() {
                    settings_screen.load_save(settings_file);
                    debug!("Loaded settings!");
                } else {
                    warn!("Failed to downcast SettingsScreen during load.");
                }
            }
            Err(e) => {
                error!("Failed to parse settings file: {}", e);
            }
        }
    } else {
        debug!("No settings file found — skipping load.");
    }
}