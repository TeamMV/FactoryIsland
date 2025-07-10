mod worldview;

use crate::gameloop::FactoryIslandClient;
use crate::input;
use crate::mods::LocalModManager;
use crate::player::ClientPlayer;
use crate::ui::display::TileSelection;
use crate::world::ClientWorld;
use api::registry::Registry;
use api::server::packets::common::{ClientDataPacket, ServerStatePacket, TileKind};
use api::server::packets::world::TileSetFromClientPacket;
use api::server::{ClientBoundPacket, ServerBoundPacket};
use api::world::tiles::pos::TilePos;
use api::world::tiles::terrain::TerrainTile;
use api::world::tiles::Orientation;
use bytebuffer::{ByteBuffer, Endian};
use log::{debug, error, warn};
use mvengine::input::consts::MouseButton;
use mvengine::input::Input;
use mvengine::modify_style;
use mvengine::net::server::ClientId;
use mvengine::rendering::RenderContext;
use mvengine::ui::elements::div::Div;
use mvengine::ui::elements::Element;
use mvengine::ui::elements::UiElementStub;
use mvengine::window::Window;
use mvengine_proc::style_expr;
use mvengine_proc::ui;
use mvutils::bytebuffer::ByteBufferExtras;
use mvutils::hashers::U64IdentityHasher;
use mvutils::once::CreateOnce;
use mvutils::save::Savable;
use mvutils::thread::ThreadSafe;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::rendering::WideRenderContext;
use mvutils::unsafe_utils::Unsafe;
use crate::game::worldview::WorldView;
use crate::gamesettings::GameSettings;
use crate::ui::display::chat::Chat;
use crate::ui::manager::{GameUiManager, UI_SETTINGS_SCREEN};
use crate::ui::settings::{SettingsScreen};

pub struct Game {
    pub conf_dir: PathBuf,
    pub client_resources: LocalModManager,
    pub settings: GameSettings,
    pub world_view: Option<WorldView>
}

impl Game {
    pub fn new() -> Self {
        let appdata = env::var("APPDATA").expect("Failed to get APPDATA environment variable");
        let mut full = PathBuf::from(appdata);
        full.push(input::PATH);

        let local_mods = LocalModManager::new();

        Self {
            conf_dir: full,
            client_resources: local_mods,
            settings: GameSettings::new(),
            world_view: None,
        }
    }
    
    pub fn load_client_res(&mut self) {
        let client_mod_path = Path::join(&self.conf_dir, "resources");
        if let Err(e) = self.client_resources.load_all(&client_mod_path) {
            error!("Error when loading client resources: {e}");
        };
    }
    
    pub fn on_frame(&mut self, window: &mut Window, client: &mut Option<FactoryIslandClient>) {
        if let Some(view) = &mut self.world_view {
            if let Some(client) = client {
                view.on_frame(window, client);
            }
        }
    }

    pub fn resize(&mut self, window: &Window) {
        if let Some(view) = &mut self.world_view {
            view.resize(window);
        }
    }
    
    pub fn on_server_state(&mut self, window: &mut Window, packet: ServerStatePacket) {
        self.world_view = Some(WorldView::new(window, packet));
        if let Some(view) = &mut self.world_view {
            view.open(window);
        }
    }

    pub fn save_settings(&self) {
        let mut file = self.configuration_directory().clone();
        file.push(crate::gamesettings::SETTINGS_FILE);
        if let Ok(mut file) = File::options().write(true).truncate(true).create(true).open(&file) {
            let mut buffer = ByteBuffer::new_le();
            self.settings.save(&mut buffer);
            let r = file.write_all(buffer.as_bytes());
            if r.is_err() {
                error!("Failed to save settings: {:?}", r.unwrap_err());
            } else {
                debug!("Saved settings!");
            }
        }
    }

    pub fn load_settings(&mut self) {
        let mut path = self.configuration_directory().clone();
        path.push(crate::gamesettings::SETTINGS_FILE);

        if let Ok(mut file) = File::open(&path) {
            let mut data = Vec::new();
            if let Err(e) = file.read_to_end(&mut data) {
                error!("Failed to read settings file: {:?}", e);
                return;
            }

            let mut buffer = ByteBuffer::from_vec(data);
            buffer.set_endian(Endian::LittleEndian);
            match GameSettings::load(&mut buffer) {
                Ok(settings) => {
                    self.settings = settings;
                    debug!("Loaded settings!");
                }
                Err(e) => {
                    error!("Failed to parse settings file: {}", e);
                }
            }
        } else {
            debug!("No settings file found — skipping load.");
        }
    }

    pub fn configuration_directory(&self) -> &PathBuf {
        &self.conf_dir
    }

    pub fn initialize(&mut self) {
        self.load_settings();
    }

    pub fn check_packet(&mut self, packet: ClientBoundPacket) {
        let this = unsafe { Unsafe::cast_lifetime(self) };
        if let Some(view) = &mut self.world_view {
            view.check_packet(packet, this);
        }
    }

    pub fn exit(&self) {
        self.save_settings();
    }
}