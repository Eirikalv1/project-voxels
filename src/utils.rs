use std::ops::RangeInclusive;

use bevy::prelude::{IVec3, Vec3};

pub const CHUNK_SIZE: f32 = 32.0;
pub const CHUNK_SIZE_MINUS_ONE: f32 = CHUNK_SIZE - 1.;
pub const CHUNK_VOLUME: usize = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize;

pub const RENDER_DISTANCE: i32 = 2;
pub const RENDER_DISTANCE_SQUARED: i32 = RENDER_DISTANCE * RENDER_DISTANCE;
pub const RENDER_DISTANCE_RANGE: RangeInclusive<i32> = -RENDER_DISTANCE..=RENDER_DISTANCE;

pub const WORLD_RADIUS: i32 = 3;
pub const WORLD_RADIUS_MINUS_ONE: i32 = WORLD_RADIUS - 1;

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

// Chunk position is the position of a chunk relative to other chunks
pub fn to_chunk_pos(pos: Vec3) -> IVec3 {
    IVec3::new(
        f32::floor(pos.x / CHUNK_SIZE) as i32,
        f32::floor(pos.y / CHUNK_SIZE) as i32,
        f32::floor(pos.z / CHUNK_SIZE) as i32,
    )
}

// World position is the is the same as Bevy's coordinate system
pub fn to_world_pos(pos: Vec3, chunk_pos: IVec3) -> Vec3 {
    pos + chunk_pos.as_vec3() * CHUNK_SIZE
}
