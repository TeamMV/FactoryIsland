pub mod tiles;
pub mod terrain_tex_mapper;
pub mod multitiles;

use crate::world::tiles::{LoadedClientTile, TileDraw};
use api::server::packets::world::{ChunkDataPacket, TileSetPacket};
use api::world::chunk::{Chunk, ToClientObject, CHUNK_TILES};
use api::world::{ChunkPos, TileSetReason, TileUnit, CHUNK_SIZE};
use mvengine::rendering::{OpenGLRenderer, RenderContext};
use mvengine::ui::geometry::SimpleRect;
use std::collections::HashMap;
use mvengine::graphics::Drawable;
use mvengine::ui::rendering::WideRenderContext;
use api::registry::ObjectSource;
use api::world::tiles::Orientation;
use api::world::tiles::pos::TilePos;
use crate::drawutils;
use crate::drawutils::Fill;
use crate::game::Game;
use crate::world::multitiles::{ClientMultiTilePlacement, CLIENT_MULTI_REG};

pub struct ClientWorld {
    loaded: HashMap<ChunkPos, ClientChunk>
}

pub struct ClientChunk {
    pub terrain: Box<[LoadedClientTile]>,
    pub tiles: Box<[Option<LoadedClientTile>]>,
    pub multitiles: Vec<ClientMultiTilePlacement>
}

impl ClientWorld {
    pub fn new() -> Self {
        Self {
            loaded: HashMap::new(),
        }
    }

    pub fn is_multitile_at(&self, pos: &TilePos) -> bool {
        for chunk_pos in pos.multitile_chunk_maybe_positions() {
            if let Some(chunk) = self.loaded.get(&chunk_pos) {
                if chunk.multitiles.iter().any(|mt| mt.includes(pos)) {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_chunk_mut(&mut self, pos: ChunkPos) -> Option<&mut ClientChunk> {
        self.loaded.get_mut(&pos)
    }

    pub fn sync(&mut self, packet: TileSetPacket, game: &Game) {
        let pos = packet.pos.chunk_pos;
        if let Some(chunk) = self.loaded.get_mut(&pos) {
            chunk.tiles[Chunk::get_index(&packet.pos)] = LoadedClientTile::from_server_tile(packet.tile, game, false);
        }
    }

    pub fn sync_chunk(&mut self, packet: ChunkDataPacket, game: &Game) {
        let terrain = (0..CHUNK_TILES)
            .map(|i| {
                LoadedClientTile::from_server_tile(packet.data.terrain[i].clone(), game, true).expect("terrain tile")
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let tiles = (0..CHUNK_TILES)
            .map(|i| {
                if let Some(obj ) = &packet.data.tiles[i] {
                    let loaded = LoadedClientTile::from_server_tile(obj.clone(), game, false);
                    loaded
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let multis = packet.data.multitiles
            .into_iter()
            .map(Into::into)
            .collect();

        let chunk = ClientChunk {
            terrain,
            tiles,
            multitiles: multis,
        };

        self.loaded.insert(packet.pos, chunk);
    }

    pub fn set_ghost_block(&mut self, pos: &TilePos, id: usize, orientation: Orientation) {
        if let Some(chunk) = self.loaded.get_mut(&pos.chunk_pos) {
            let index = Chunk::get_index(&pos);
            if id == 0 {
                chunk.tiles[index] = None;
            } else {
                let loaded = LoadedClientTile::new_ghost(id, orientation);
                chunk.tiles[index] = Some(loaded);
            }
        }
    }
    
    pub fn drop_chunk(&mut self, pos: ChunkPos) {
        self.loaded.remove(&pos);
    }

    pub fn drop_all(&mut self) {
        self.loaded.clear();
    }

    pub fn draw(&self, renderer: &mut impl WideRenderContext, view_area: &SimpleRect, tile_size: i32) {
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
                        if self.is_multitile_at(&pos) {
                            continue;
                        }
                        if let Some(tile) = &chunk.tiles[i] {
                            if let Some(drawer) = tile.drawer {
                                drawer(renderer, view_area, &pos, tile_size, tile);
                            } else {
                                let orientation = tile.orientation;
                                tile.draw(renderer, tile_size, &pos, orientation, view_area, terrain_height - 101);
                            }
                        }
                    }
                }
                for multitile in &chunk.multitiles {
                    let terrain = &chunk.terrain[Chunk::get_index(&multitile.pos)];
                    let terrain_height = 1000 - terrain.id as i32 * 100;
                    let tex = if let Some(client_mt) = &multitile.client_multi_tile {
                        client_mt.get_relevant_texture(multitile.extent.0 > multitile.extent.1)
                    } else {
                        Drawable::missing()
                    };
                    let w = multitile.extent.0 as f64;
                    let h = multitile.extent.1 as f64;
                    drawutils::draw_in_world(renderer, view_area, multitile.pos.to_unit(), (w, h), Fill::Drawable(tex, Orientation::North), tile_size, (terrain_height - 101) as f32);
                }
            }
        }
    }
}