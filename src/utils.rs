use bevy::prelude::{IVec3, Vec3};

pub const CHUNK_SIZE: f32 = 32.0;
pub const CHUNK_SIZE_MINUS_ONE: f32 = CHUNK_SIZE - 1.;
pub const CHUNK_VOLUME: usize = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize;

pub type ChunkData = Box<[VoxelVisibility; CHUNK_VOLUME]>;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VoxelVisibility {
    Empty,
    Transparent,
    Opaque,
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

pub fn to_ivec3(pos: Vec3) -> IVec3 {
    IVec3::new(pos.x as i32, pos.y as i32, pos.z as i32)
}

pub fn to_fvec3(pos: IVec3) -> Vec3 {
    Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32)
}
