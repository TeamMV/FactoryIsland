use crate::gameloop::FactoryIslandClient;
use crate::input;
use crate::player::ClientPlayer;
use crate::world::ClientWorld;
use mvengine::input::Input;
use mvengine::net::server::ClientId;
use mvengine::rendering::RenderContext;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use bytebuffer::ByteBuffer;
use log::error;
use mvutils::bytebuffer::ByteBufferExtras;
use mvutils::hashers::U64IdentityHasher;
use mvutils::once::CreateOnce;
use mvutils::save::Savable;
use api::registry::Registry;
use api::server::packets::common::{ClientDataPacket, ServerStatePacket};
use api::world::tiles::terrain::TerrainTile;
use crate::mods::LocalModManager;

pub struct Game {
    pub world: ClientWorld,
    pub player: ClientPlayer,
    pub other_players: HashMap<ClientId, ClientPlayer, U64IdentityHasher>,
    pub conf_dir: PathBuf,
    pub client_resources: LocalModManager,
    pub(crate) tile_size: i32,
}

impl Game {
    pub fn new() -> Self {
        let appdata = env::var("APPDATA").expect("Failed to get APPDATA environment variable");
        let mut full = PathBuf::from(appdata);
        full.push(input::PATH);

        let local_mods = LocalModManager::new();

        Self {
            world: ClientWorld::new(),
            player: ClientPlayer::new(1, 1, ClientDataPacket {
                name: "v22".to_string(),
                render_distance: 1,
            }),
            other_players: HashMap::with_hasher(U64IdentityHasher::default()),
            conf_dir: full,
            client_resources: local_mods,
            tile_size: 50,
        }
    }
    
    pub fn load_client_res(&mut self) {
        let client_mod_path = Path::join(&self.conf_dir, "resources");
        if let Err(e) = self.client_resources.load_all(&client_mod_path) {
            error!("Error when loading client resources: {e}");
        };
    }
    
    pub fn on_server_state(&mut self, packet: ServerStatePacket) {
        
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.player.resize_view(width, height);
    }

    pub fn player_join(&mut self, player: ClientPlayer, id: ClientId) {
        self.other_players.insert(id, player);
    }

    pub fn player_leave(&mut self, id: ClientId) {
        self.other_players.remove(&id);
    }

    pub fn draw_world(&mut self, ctx: &mut impl RenderContext) {
        self.world.draw(ctx, &self.player.camera.view_area, self.tile_size);
    }
    
    pub fn draw_players(&mut self, ctx: &mut impl RenderContext) {
        for player in self.other_players.values() {
            player.draw_from_other_pov(ctx, &self.player.camera.view_area, self.tile_size);
        }

        self.player.draw(ctx, self.tile_size);
    }

    pub fn check_inputs(&mut self, input: &Input, client: &mut FactoryIslandClient) {
        let mut has_moved = false;
        let speed = 0.4;
        if input.is_action(input::MOVE_FORWARD) {
            self.player.move_by((0.0, speed), self.tile_size);
            has_moved = true;
        }
        if input.is_action(input::MOVE_BACK) {
            self.player.move_by((0.0, -speed), self.tile_size);
            has_moved = true;
        }
        if input.is_action(input::MOVE_LEFT) {
            self.player.move_by((-speed, 0.0), self.tile_size);
            has_moved = true;
        }
        if input.is_action(input::MOVE_RIGHT) {
            self.player.move_by((speed, 0.0), self.tile_size);
            has_moved = true;
        }
        if has_moved {
            self.player.broadcast_position(client);
        }
    }

    pub fn configuration_directory(&self) -> &PathBuf {
        &self.conf_dir
    }
}