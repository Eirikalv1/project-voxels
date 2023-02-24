use bevy::prelude::{IVec3, Vec3};

use crate::utils::{CHUNK_SIZE, WORLD_RADIUS_MINUS_ONE};

pub fn get_chunk_rotation(chunk_pos: IVec3) -> IVec3 {
    IVec3::new(
        (chunk_pos.x >= WORLD_RADIUS_MINUS_ONE) as i32 - (chunk_pos.x <= -WORLD_RADIUS_MINUS_ONE) as i32,
        (chunk_pos.y >= WORLD_RADIUS_MINUS_ONE) as i32 - (chunk_pos.y <= -WORLD_RADIUS_MINUS_ONE) as i32,
        (chunk_pos.z >= WORLD_RADIUS_MINUS_ONE) as i32 - (chunk_pos.z <= -WORLD_RADIUS_MINUS_ONE) as i32,
    )
}

pub fn rotate_chunk(chunk_rot: IVec3, mut pos: Vec3) -> Vec3 {
    if chunk_rot.x == 0 && chunk_rot.y != -1 && chunk_rot.z == 0 {
        return pos;
    }

    match chunk_rot {
        IVec3 { x: 0, y: -1, z: 0 } => pos.y = -pos.y - 1.,
        IVec3 { x: 1, y: 0, z: 0 } => (pos.x, pos.y) = (pos.y, pos.x),
        IVec3 { x: -1, y: 0, z: 0 } => (pos.x, pos.y) = (-pos.y - 1., pos.x + 1.),
        IVec3 { x: 0, y: 0, z: 1 } => (pos.z, pos.y) = (pos.y, pos.z),
        IVec3 { x: 0, y: 0, z: -1 } => (pos.z, pos.y) = (-pos.y + CHUNK_SIZE, pos.z),
        _ => unreachable!("Chunk rotation out of range"),
    }
    pos
}
