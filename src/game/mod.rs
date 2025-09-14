pub mod persistent;
pub mod place_tile;
pub mod worldview;

use crate::game::persistent::{PersistentGameData, PersistentLoadedData, PERSISTENT_FILE};
use crate::game::worldview::WorldView;
use crate::gameloop::FactoryIslandClient;
use crate::gamesettings::{GameSettings, SETTINGS_FILE};
use crate::input;
use crate::player::ClientPlayer;
use crate::ui::display::chat::Chat;
use crate::ui::display::TileSelection;
use crate::ui::manager::{GameUiManager, UI_SETTINGS_SCREEN};
use crate::ui::settings::SettingsScreen;
use crate::world::ClientWorld;
use api::player::profile::PlayerProfile;
use api::registry::Registry;
use api::server::packets::common::{ClientDataPacket, ServerStatePacket, TileKind};
use api::server::packets::world::TileSetFromClientPacket;
use api::server::{ClientBoundPacket, ServerBoundPacket};
use api::world::tiles::pos::TilePos;
use api::world::tiles::terrain::TerrainTile;
use api::world::tiles::Orientation;
use bytebuffer::{ByteBuffer, Endian};
use log::{debug, error, warn};
use mvengine::game::fs::smartdir::SmartDir;
use mvengine::input::consts::MouseButton;
use mvengine::input::Input;
use mvengine::modify_style;
use mvengine::net::server::ClientId;
use mvengine::rendering::RenderContext;
use mvengine::ui::elements::div::Div;
use mvengine::ui::elements::events::UiClickAction;
use mvengine::ui::elements::Element;
use mvengine::ui::elements::UiElementStub;
use mvengine::ui::rendering::WideRenderContext;
use mvengine::window::Window;
use mvengine_proc::style_expr;
use mvengine_proc::ui;
use mvutils::bytebuffer::ByteBufferExtras;
use mvutils::hashers::U64IdentityHasher;
use mvutils::once::CreateOnce;
use mvutils::save::Savable;
use mvutils::thread::ThreadSafe;
use mvutils::unsafe_utils::Unsafe;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub const INTERNAL_IP: &str = "127.0.0.1:4040";

pub struct Game {
    pub conf_dir: SmartDir,
    pub res_dir: SmartDir,
    pub settings: GameSettings,
    pub world_view: Option<WorldView>,
    pub profile: PlayerProfile,
    pub is_internal: bool,
    pub persistent_game_data: PersistentLoadedData,
}

impl Game {
    pub fn new(is_internal: bool) -> Self {
        let appdata = env::var("APPDATA").expect("Failed to get APPDATA environment variable");
        let mut full = PathBuf::from(appdata);
        full.push(input::PATH);

        let conf_dir = SmartDir::new(full);
        let res_dir = conf_dir.join("resources");

        let mut persistent_game_data = conf_dir
            .read_object::<PersistentGameData>(PERSISTENT_FILE)
            .unwrap_or(PersistentGameData::new());

        let profile = PlayerProfile::load_or_create(&conf_dir);

        Self {
            conf_dir,
            res_dir,
            settings: GameSettings::new(),
            world_view: None,
            profile,
            is_internal,
            persistent_game_data: persistent_game_data.to_loaded(),
        }
    }

    pub fn on_frame(
        &mut self,
        window: &mut Window,
        client: &mut Option<FactoryIslandClient>,
        ui_manager: &mut GameUiManager,
    ) {
        if let Some(view) = &mut self.world_view {
            if let Some(client) = client {
                view.on_frame(window, client, ui_manager);
            }
        }
    }

    pub fn resize(&mut self, window: &Window) {
        if let Some(view) = &mut self.world_view {
            view.resize(window);
        }
    }

    pub fn on_server_state(&mut self, window: &mut Window, packet: ServerStatePacket) {
        self.world_view = Some(WorldView::new(window, packet, self));
        if let Some(view) = &mut self.world_view {
            view.open(window);
        }
    }

    pub fn save_settings(&self) {
        let dir = self.configuration_directory();
        if let Some(_) = dir.save_object(&self.settings, SETTINGS_FILE) {
            debug!("Saved settings!");
        }
        let p_data = PersistentGameData::from_loaded(&self.persistent_game_data);
        if let Some(_) = dir.save_object(&p_data, PERSISTENT_FILE) {
            debug!("Saved persistent data!");
        }
    }

    pub fn load_settings(&mut self) {
        let dir = self.configuration_directory();

        if let Some(settings) = dir.read_object::<GameSettings>(SETTINGS_FILE) {
            self.settings = settings;
            debug!("Loaded settings!");
        } else {
            debug!("No settings file found — skipping load.");
        }
    }

    pub fn configuration_directory(&self) -> &SmartDir {
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
