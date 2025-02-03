use crate::game::world::chunk::Chunk;

pub struct ChunkLoadEvent {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

pub struct ChunkGenerateEvent {
    pub chunk: &'static Chunk
}