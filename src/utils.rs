use bevy::prelude::Vec3;

pub const CHUNK_SIZE: f32 = 32.0;
pub const CHUNK_VOLUME: usize = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize;

pub type ChunkData = Box<[VoxelType; CHUNK_VOLUME]>;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VoxelType {
    Air,
    Block,
}

pub fn to_3d(pos: f32) -> Vec3 {
    Vec3::new(
        pos % CHUNK_SIZE,
        ((pos / CHUNK_SIZE) as i32 % CHUNK_SIZE as i32) as f32,
        (pos / (CHUNK_SIZE * CHUNK_SIZE)) as i32 as f32,
    )
}

pub fn to_1d(x: f32, y: f32, z: f32) -> usize {
    ((z * CHUNK_SIZE * CHUNK_SIZE) + (y * CHUNK_SIZE) + x) as usize
}
