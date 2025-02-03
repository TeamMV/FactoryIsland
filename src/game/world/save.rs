use crate::game::world::chunk::Chunk;
use bytebuffer::ByteBuffer;
use crossbeam_channel::{Receiver, Sender};
use mvutils::save::Savable;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::thread::JoinHandle;
use std::{env, thread};
use log::error;

pub const CONFIGURATION_PATH: &str = ".factoryisland";

pub struct ChunkSaver;

impl ChunkSaver {
    pub fn save_chunk(&self, chunk: Chunk) {
        let appdata = env::var("APPDATA").expect("Failed to get APPDATA environment variable");
        let mut full = PathBuf::from(appdata);
        full.push(CONFIGURATION_PATH);
        std::fs::create_dir_all(&full).expect("Failed to create configuration directory");

        let filename = format!("c{}_{}.chunk", chunk.chunk_world_x, chunk.chunk_world_z);
        full.push(&filename);

        let mut file = File::options().write(true).open(&full);
        if let Err(_) = file {
            file = File::create(&full);
        }
        if let Ok(mut file) = file {
            let mut buffer = ByteBuffer::new();
            chunk.save(&mut buffer);
            file.write_all(buffer.as_bytes()).expect("Failed to write to file");
            return;
        }
        error!("failed to create or open file: {:?} in {:?}", file, full);
    }

    pub fn load_chunk(&self, cx: i32, cy: i32, seed: u32) -> Chunk {
        let appdata = env::var("APPDATA").expect("Failed to get APPDATA environment variable");
        let mut full = PathBuf::from(appdata);
        full.push(CONFIGURATION_PATH);

        let filename = format!("c{}_{}.chunk", cx, cy);
        full.push(&filename);

        let mut file = File::options().read(true).open(&full);
        if let Err(_) = file {
            return Chunk::new(cx, cy, seed);
        }
        if let Ok(mut file) = file {
            let mut buffer = Vec::new();
            if let Err(_) = file.read_to_end(&mut buffer) {
                return Chunk::new(cx, cy, seed);
            }
            let mut buffer = ByteBuffer::from(buffer);
            return Chunk::load(&mut buffer).unwrap_or(Chunk::new(cx, cy, seed));
        }
        unreachable!()
    }
}

pub enum ChunkTask {
    Load(i32, i32, u32, Box<dyn FnOnce(Chunk) + Send + 'static>),
    Save(Chunk),
}

pub struct ChunkSaverThread {
    sender: Sender<ChunkTask>,
    handle: Option<JoinHandle<()>>,
}

impl ChunkSaverThread {
    pub fn new(saver: ChunkSaver) -> Self {
        let (sender, receiver): (Sender<ChunkTask>, Receiver<ChunkTask>) = crossbeam_channel::unbounded();

        let handle = thread::spawn(move || {
            let receiver = receiver;
            let saver = saver;

            while let Ok(task) = receiver.recv() {
                match task {
                    ChunkTask::Load(cx, cy, seed, callback) => {
                        let chunk = saver.load_chunk(cx, cy, seed);
                        callback(chunk);
                    }
                    ChunkTask::Save(chunk) => {
                        saver.save_chunk(chunk);
                    }
                }
            }
        });

        Self {
            sender,
            handle: Some(handle),
        }
    }

    pub fn request(&self, task: ChunkTask) {
        self.sender.send(task).expect("Failed to send task");
    }

    pub fn stop(self) {
        drop(self.sender);
        if let Some(handle) = self.handle {
            handle.join().expect("Failed to join worker thread");
        }
    }
}