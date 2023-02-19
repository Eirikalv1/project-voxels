use bevy::prelude::Vec3;

pub const CHUNK_SIZE: f32 = 32.0;
pub const CHUNK_SIZE_MINUS_ONE: f32 = CHUNK_SIZE - 1.;
pub const CHUNK_VOLUME: usize = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize;

pub type ChunkData = Box<[VoxelType; CHUNK_VOLUME]>;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VoxelType {
    Air,
    Block,
}

pub fn to_3d(pos: usize) -> Vec3 {
    Vec3::new(
        pos as f32 % CHUNK_SIZE,
        ((pos as f32 / CHUNK_SIZE) as i32 % CHUNK_SIZE as i32) as f32,
        (pos as f32 / (CHUNK_SIZE * CHUNK_SIZE)) as i32 as f32,
    )
}

pub fn to_1d(x: f32, y: f32, z: f32) -> usize {
    ((z * CHUNK_SIZE * CHUNK_SIZE) + (y * CHUNK_SIZE) + x) as usize
}

pub fn tuple_to_vec3(pos: (i32, i32, i32)) -> Vec3 {
    Vec3::new(pos.0 as f32, pos.1 as f32, pos.2 as f32)
}

pub fn vec3_to_tuple(pos: Vec3) -> (i32, i32, i32) {
    (pos.x as i32, pos.y as i32, pos.z as i32)
}