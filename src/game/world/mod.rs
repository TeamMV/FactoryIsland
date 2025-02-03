use crate::game::camera::Camera;
use crate::game::event::EventDispatcher;
use crate::game::world::chunk::{Chunk, TilePos, CHUNK_SIZE, RENDER_DISTANCE};
use crate::game::world::generator::default_generator;
use crate::game::world::save::{ChunkSaver, ChunkSaverThread, ChunkTask};
use crate::game::world::tiles::{Tile, TILE_SIZE};
use hashbrown::HashSet;
use itertools::Itertools;
use mvengine::rendering::control::RenderController;
use mvsync::{MVSync, MVSyncSpecs};
use mvutils::unsafe_utils::Unsafe;
use parking_lot::Mutex;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

pub mod tiles;
pub mod chunk;
pub mod generator;
mod save;
mod render;

pub struct World {
    seed: u32,
    loaded_chunks: Arc<Mutex<Vec<Chunk>>>,
    loading_chunks: Arc<Mutex<HashSet<(i32, i32)>>>,
    saver: ChunkSaverThread,
    sync: Arc<MVSync>
}

impl World {
    pub fn create(seed: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let seed = hasher.finish() as u32;

        let mut specs = MVSyncSpecs::default();
        specs.thread_count = 1;
        specs.workers_per_thread = 1;
        let sync = MVSync::new(specs);

        Self {
            seed,
            loaded_chunks: Arc::new(Mutex::new(vec![])),
            sync: sync.clone(),
            saver: ChunkSaverThread::new(ChunkSaver),
            loading_chunks: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn get_chunk(&mut self, pos: &TilePos) -> &Chunk {
        let chunk_x = pos.world_chunk_x;
        let chunk_z = pos.world_chunk_z;

        let mut idx = None;
        let mut guard = self.loaded_chunks.lock();
        for (i, chunk) in guard.iter().enumerate() {
            if chunk.chunk_world_x == chunk_x && chunk.chunk_world_z == chunk_z {
                idx = Some(i);
                break;
            }
        }

        if idx.is_none() {
            let mut c = Chunk::new(chunk_x, chunk_z, self.seed);
            c.request_generate(default_generator);
            guard.push(c);
            idx = Some(guard.len() - 1);
        }

        let val = &guard[idx.unwrap()];
        let val = unsafe { Unsafe::cast_static(val) };
        drop(guard);

        val
    }

    pub fn get_chunk_mut(&mut self, pos: &TilePos) -> &mut Chunk {
        let chunk_x = pos.world_chunk_x;
        let chunk_z = pos.world_chunk_z;

        let mut guard = self.loaded_chunks.lock();

        let mut idx = None;
        for (i, chunk) in guard.iter_mut().enumerate() {
            if chunk.chunk_world_x == chunk_x && chunk.chunk_world_z == chunk_z {
                idx = Some(i);
                break;
            }
        }

        if idx.is_none() {
            let mut c = Chunk::new(chunk_x, chunk_z, self.seed);
            c.request_generate(default_generator);
            guard.push(c);
            idx = Some(guard.len() - 1);
        }

        let idx = idx.unwrap();
        let val = &mut guard[idx];
        let val = unsafe { Unsafe::cast_mut_static(val) };
        drop(guard);
        val
    }

    pub fn set_tile_at(&mut self, tile: Tile, pos: TilePos) {
        let target_chunk = self.get_chunk_mut(&pos);
        target_chunk.set_tile_at(tile, pos);
    }

    pub fn get_tile_at(&mut self, pos: TilePos) -> &Tile {
        let target_chunk = self.get_chunk(&pos);
        target_chunk.get_tile_at(pos)
    }

    pub fn get_tile_at_mut(&mut self, pos: TilePos) -> &mut Tile {
        let target_chunk = self.get_chunk_mut(&pos);
        target_chunk.get_tile_at_mut(pos)
    }

    pub fn get_y_level(&mut self, pos: TilePos) -> i32 {
        let target_chunk = self.get_chunk(&pos);
        target_chunk.get_y_level(pos)
    }

    pub fn chunk_is_loaded(&self, chunk_x: i32, chunk_z: i32) -> bool {
        let guard = self.loaded_chunks.lock();
        guard.iter().any(|chunk| chunk.chunk_world_x == chunk_x && chunk.chunk_world_z == chunk_z)
    }

    pub fn draw(&self, controller: &mut RenderController, camera: &Camera) {
        let mut guard = self.loaded_chunks.lock();
        for chunk in guard.iter() {
            chunk.draw_tiles(controller, camera);
        }
    }

    pub fn on_cam_move(&mut self, camera: &Camera, event_dispatcher: &mut EventDispatcher) {
        let this = unsafe { Unsafe::cast_mut_static(self) };

        let nearest_chunk_x = -(camera.x / CHUNK_SIZE as f64).floor() as i32;
        let nearest_chunk_z = -(camera.y / CHUNK_SIZE as f64).floor() as i32;

        let mut to_unload = vec![];

        {
            let guard = self.loaded_chunks.lock();

            for chunk in guard.iter() {
                if chunk.chunk_world_x < nearest_chunk_x - RENDER_DISTANCE
                    || chunk.chunk_world_x > nearest_chunk_x + RENDER_DISTANCE
                    || chunk.chunk_world_z < nearest_chunk_z - RENDER_DISTANCE
                    || chunk.chunk_world_z > nearest_chunk_z + RENDER_DISTANCE
                {
                    to_unload.push(chunk.clone());
                }
            }
        }

        for chunk in to_unload.into_iter() {
            this.unload_chunk(chunk);
        }

        for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
                let rx = x + nearest_chunk_x;
                let rz = z + nearest_chunk_z;
                let guard = self.loading_chunks.lock();
                if !guard.contains(&(rx, rz)) {
                    drop(guard);
                    self.load_chunk(rx, rz);
                    //event_dispatcher.dispatch(GameEvent::ChunkLoad(ChunkLoadEvent { chunk_x: rx, chunk_z: rz }));
                } else {
                    drop(guard);
                }
            }
        }
    }

    pub fn load_chunk(&mut self, chunk_x: i32, chunk_z: i32) {
        let mut guard = self.loaded_chunks.lock();
        for chunk in guard.iter() {
            if chunk.chunk_world_x == chunk_x && chunk.chunk_world_z == chunk_z {
                return;
            }
        }

        drop(guard);

        let arc = self.loaded_chunks.clone();
        let arc2 = self.loading_chunks.clone();

        let mut lguard = self.loading_chunks.lock();
        lguard.insert((chunk_x, chunk_z));
        drop(lguard);
        self.saver.request(ChunkTask::Load(chunk_x, chunk_z, self.seed, Box::new(move |mut chunk| {
            let arc2 = arc2.clone();
            let mut lguard = arc2.lock();
            lguard.remove(&(chunk_x, chunk_z));
            drop(lguard);
            let arc = arc.clone();
            chunk.request_generate(default_generator);
            let mut guard = arc.lock();
            guard.push(chunk);
        })));
    }

    pub fn unload_chunk(&mut self, chunk: Chunk) {
        let mut guard = self.loaded_chunks.lock();
        let mut to_remove = vec![];
        for (idx, other) in guard.iter().enumerate() {
            if chunk.chunk_world_x == other.chunk_world_x && chunk.chunk_world_z == other.chunk_world_z {
                to_remove.push(idx);
            }
        }
        let mut off = 0;
        for idx in to_remove.into_iter().sorted() {
            let removed = guard.remove(idx - off);
            off += 1;
            self.saver.request(ChunkTask::Save(removed));
        }
    }

    pub fn screen_to_world_pos(sx: i32, sy: i32, camera: &Camera) -> (i32, i32) {
        unsafe {
            let ox = sx as f64 - (camera.x * TILE_SIZE as f64);
            let oy = sy as f64 - (camera.y * TILE_SIZE as f64);

            let wx = (ox / TILE_SIZE as f64).floor() as i32;
            let wy = (oy / TILE_SIZE as f64).floor() as i32;

            (wx, wy)
        }
    }
}