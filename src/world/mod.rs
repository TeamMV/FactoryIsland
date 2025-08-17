pub mod tiles;
pub mod terrain_tex_mapper;
pub mod tile_tex_mapper;

use crate::world::tiles::{ClientTile, TileDraw};
use api::server::packets::world::{ChunkDataPacket, TileSetPacket};
use api::world::chunk::{Chunk, CHUNK_TILES};
use api::world::{ChunkPos, CHUNK_SIZE};
use mvengine::rendering::RenderContext;
use mvengine::ui::geometry::SimpleRect;
use std::collections::HashMap;
use crate::game::Game;

pub struct ClientWorld {
    loaded: HashMap<ChunkPos, ClientChunk>
}

pub struct ClientChunk {
    terrain: Box<[ClientTile]>,
    tiles: Box<[Option<ClientTile>]>
}

impl ClientWorld {
    pub fn new() -> Self {
        Self {
            loaded: HashMap::new(),
        }
    }

    pub fn sync(&mut self, packet: TileSetPacket, game: &Game) {
        let tile = ClientTile::from_server_tile(packet.tile, game, false);
        let pos = (packet.pos.chunk_pos.x, packet.pos.chunk_pos.z);
        if let Some(chunk) = self.loaded.get_mut(&pos) {
            chunk.tiles[Chunk::get_index(&packet.pos)] = Some(tile);
        }
    }

    pub fn sync_chunk(&mut self, packet: ChunkDataPacket, game: &Game) {
        let terrain = (0..CHUNK_TILES)
            .map(|i| ClientTile::from_server_tile(packet.data.terrain[i].clone(), game, true))
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let tiles = (0..CHUNK_TILES)
            .map(|i| {
                packet.data.tiles[i].as_ref().map(|obj| {
                    ClientTile::from_server_tile(obj.clone(), game, false)
                })
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let chunk = ClientChunk {
            terrain,
            tiles,
        };

        self.loaded.insert(packet.pos, chunk);
    }
    
    pub fn drop_chunk(&mut self, pos: ChunkPos) {
        self.loaded.remove(&pos);
    }

    pub fn drop_all(&mut self) {
        self.loaded.clear();
    }

    pub fn draw(&self, renderer: &mut impl RenderContext, view_area: &SimpleRect, tile_size: i32) {
        for (pos, chunk) in self.loaded.iter() {
            let chunk_area = SimpleRect::new(pos.0 * CHUNK_SIZE * tile_size, pos.1 * CHUNK_SIZE * tile_size, CHUNK_SIZE * tile_size, CHUNK_SIZE * tile_size);
            if view_area.intersects(&chunk_area) {
                for i in 0..chunk.terrain.len() {
                    let terrain = &chunk.terrain[i];
                    let orientation = terrain.orientation;
                    let pos = Chunk::position_from_index(pos, i);
                    let tile_rect = SimpleRect::new(pos.raw.0 * tile_size, pos.raw.1 * tile_size, tile_size, tile_size);
                    if view_area.intersects(&tile_rect) {
                        let terrain_height = 1000 - terrain.id as i32 * 100;
                        terrain.draw(renderer, tile_size, &pos, orientation, view_area, terrain_height);
                        if let Some(tile) = &chunk.tiles[i] {
                            let orientation = tile.orientation;
                            tile.draw(renderer, tile_size, &pos, orientation, view_area, terrain_height - 101);
                        }
                    }
                }
            }
        }
    }
}