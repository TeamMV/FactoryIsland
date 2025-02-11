use crate::game::camera::Camera;
use crate::game::world::chunk::{Chunk, TilePos, CHUNK_SIZE, RENDER_DISTANCE, UNLOAD_DISTANCE};
use crate::game::world::generator::GeneratorPipeline;
use crate::game::world::save::{ChunkSaver, ChunkSaverThread, ChunkTask};
use crate::game::world::tiles::{Tile, TILE_SIZE};
use hashbrown::HashSet;
use itertools::Itertools;
use mvengine::rendering::control::RenderController;
use mvutils::unsafe_utils::Unsafe;
use parking_lot::Mutex;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use crate::WINDOW_SIZE;

pub mod tiles;
pub mod chunk;
pub mod generator;
pub mod save;
pub mod render;

pub struct World {
    seed: u32,
    loaded_chunks: Arc<Mutex<Vec<Chunk>>>,
    loading_chunks: Arc<Mutex<HashSet<(i32, i32)>>>,
    saver: ChunkSaverThread,
    generator: Arc<Mutex<GeneratorPipeline>>,
}

impl World {
    pub fn create(seed: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let seed = hasher.finish() as u32;

        Self {
            seed,
            loaded_chunks: Arc::new(Mutex::new(vec![])),
            saver: ChunkSaverThread::new(ChunkSaver),
            loading_chunks: Arc::new(Mutex::new(HashSet::new())),
            generator: Arc::new(Mutex::new(GeneratorPipeline::new(seed))),
        }
    }

    fn generate_chunk(&self, chunk: &mut Chunk) {
        let mut lock = self.generator.lock();
        lock.generate_chunk(chunk);
    }

    pub fn set_tile_at(&mut self, pos: TilePos, tile: Tile) {
        let target_chunk = self.force_load(pos.world_chunk_x, pos.world_chunk_z);
        target_chunk.set_tile_at(tile, pos);
    }

    pub fn get_tile_at(&mut self, pos: TilePos) -> &Tile {
        let target_chunk = self.force_load(pos.world_chunk_x, pos.world_chunk_z);
        target_chunk.get_tile_at(pos)
    }

    pub fn get_tile_at_mut(&mut self, pos: TilePos) -> &mut Tile {
        let target_chunk = self.force_load(pos.world_chunk_x, pos.world_chunk_z);
        target_chunk.get_tile_at_mut(pos)
    }

    pub fn chunk_is_loaded(&self, chunk_x: i32, chunk_z: i32) -> bool {
        let guard = self.loaded_chunks.lock();
        guard.iter().any(|chunk| chunk.chunk_world_x == chunk_x && chunk.chunk_world_z == chunk_z)
    }

    pub fn draw(&self, controller: &mut RenderController, camera: &Camera) {
        let mut guard = self.loaded_chunks.lock();
        unsafe {
            let (x, z) = World::screen_to_world_pos(0, 0, camera);
            let tile_pos = TilePos::new(x, z);
            let (x, z) = (tile_pos.world_chunk_x, tile_pos.world_chunk_z);
            for chunk in guard.iter() {
                if chunk.chunk_world_x < x + RENDER_DISTANCE + 1 && chunk.chunk_world_x > x - RENDER_DISTANCE {
                    if chunk.chunk_world_z < z + RENDER_DISTANCE + 1 && chunk.chunk_world_z > z - RENDER_DISTANCE {
                        chunk.draw_tiles(controller, camera);
                    }
                }
            }
        }
    }

    pub fn on_cam_move(&mut self, camera: &Camera) {
        let this = unsafe { Unsafe::cast_mut_static(self) };

        let nearest_chunk_x = -(camera.x / CHUNK_SIZE as f64).floor() as i32;
        let nearest_chunk_z = -(camera.y / CHUNK_SIZE as f64).floor() as i32;

        let mut to_unload = vec![];

        {
            let guard = self.loaded_chunks.lock();

            for chunk in guard.iter() {
                if chunk.chunk_world_x < nearest_chunk_x - UNLOAD_DISTANCE
                    || chunk.chunk_world_x > nearest_chunk_x + UNLOAD_DISTANCE
                    || chunk.chunk_world_z < nearest_chunk_z - UNLOAD_DISTANCE
                    || chunk.chunk_world_z > nearest_chunk_z + UNLOAD_DISTANCE
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

        let seed = self.seed;
        let generator = self.generator.clone();
        self.saver.request(ChunkTask::Load(chunk_x, chunk_z, self.seed, Box::new(move |mut chunk| unsafe {
            if let Some(mut chunk) = chunk {
                let arc2 = arc2.clone();
                let mut lguard = arc2.lock();
                lguard.remove(&(chunk_x, chunk_z));
                drop(lguard);
                let mut generator_lock = generator.lock();
                generator_lock.generate_chunk(&mut chunk);
                drop(generator_lock);
                let mut lock = arc.lock();
                lock.push(chunk);
            } else {
                let mut chunk = Chunk::new(chunk_x, chunk_z, seed);
                let mut generator_lock = generator.lock();
                generator_lock.generate_chunk(&mut chunk);
                drop(generator_lock);
                let arc = arc.clone();
                let mut lock = arc.lock();
                lock.push(chunk);
            }
        })));
    }

    pub fn force_load(&mut self, chunk_x: i32, chunk_z: i32) -> &mut Chunk {
        let mut guard = self.loaded_chunks.lock();

        for chunk in guard.iter_mut() {
            if chunk.chunk_world_x == chunk_x && chunk.chunk_world_z == chunk_z {
                return unsafe { Unsafe::cast_mut_static(chunk) };
            }
        }

        drop(guard);

        if let Some(mut chunk) = self.saver.load_now(chunk_x, chunk_z, self.seed) {
            self.generate_chunk(&mut chunk);
            let mutref = unsafe { Unsafe::cast_mut_static(&mut chunk) };
            let mut lock = self.loaded_chunks.lock();
            lock.push(chunk);
            drop(lock);
            mutref
        } else {
            let mut chunk = Chunk::new(chunk_x, chunk_z, self.seed);
            self.generate_chunk(&mut chunk);
            let mut lock = self.loaded_chunks.lock();
            lock.push(chunk);
            let c = lock.last_mut().unwrap();
            let c = unsafe { Unsafe::cast_mut_static(c) };
            drop(lock);
            c
        }
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