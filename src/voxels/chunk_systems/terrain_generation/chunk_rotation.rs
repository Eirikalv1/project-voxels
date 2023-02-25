use bevy::prelude::{IVec3, Vec3};

use crate::utils::{CHUNK_SIZE, WORLD_RADIUS_MINUS_ONE, WORLD_DIAMETER};

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
        IVec3::NEG_Y => (pos.z, pos.y) = (pos.z + 1., -pos.y - 1.),
        IVec3::X => (pos.x, pos.y) = (pos.y, pos.x),
        IVec3::NEG_X => (pos.x, pos.y) = (-pos.y - 1., pos.x + 1.),
        IVec3::Z => (pos.z, pos.y) = (pos.y, pos.z),
        IVec3::NEG_Z => (pos.z, pos.y) = (-pos.y + CHUNK_SIZE, pos.z),
        _ => unreachable!("Chunk rotation out of range"),
    }
    pos
}

pub fn chunk_pos_flattend_horistonally(chunk_pos: IVec3, chunk_rot: IVec3) -> IVec3 {
    match chunk_rot {
        IVec3::X | IVec3::NEG_X => IVec3::new(chunk_pos.x * WORLD_DIAMETER, chunk_pos.y, chunk_pos.z),
        IVec3::Y => chunk_pos,
        IVec3::NEG_Y => IVec3::new(chunk_pos.x * 2 * WORLD_DIAMETER, chunk_pos.y, chunk_pos.z),
        IVec3::Z | IVec3::NEG_Z => IVec3::new(chunk_pos.x, chunk_pos.y, chunk_pos.z * WORLD_DIAMETER),
        _ => unreachable!("Chunk rotation out of range"),
    }
}