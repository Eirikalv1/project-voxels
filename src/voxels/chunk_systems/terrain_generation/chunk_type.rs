use bevy::prelude::IVec3;

use crate::utils::WORLD_RADIUS_MINUS_ONE;

#[derive(Clone, PartialEq, Eq)]
pub enum ChunkType {
    Center,
    Edge,
    Corner,
    Inside,
    Outside,
}

impl ChunkType {
    pub fn get_chunk_type(chunk_pos: IVec3, chunk_rot: IVec3) -> Self {
        if chunk_pos.x > WORLD_RADIUS_MINUS_ONE
            || chunk_pos.y > WORLD_RADIUS_MINUS_ONE
            || chunk_pos.z > WORLD_RADIUS_MINUS_ONE
            || chunk_pos.x < -WORLD_RADIUS_MINUS_ONE
            || chunk_pos.y < -WORLD_RADIUS_MINUS_ONE
            || chunk_pos.z < -WORLD_RADIUS_MINUS_ONE
        {
            return Self::Outside;
        }

        match chunk_rot {
            IVec3::ZERO => Self::Inside,

            IVec3::X => Self::Center,
            IVec3::NEG_X => Self::Center,
            IVec3::Y => Self::Center,
            IVec3::NEG_Y => Self::Center,
            IVec3::Z => Self::Center,
            IVec3::NEG_Z => Self::Center,

            IVec3 { x: 1, y: 1, z: 0 } => Self::Edge,
            IVec3 { x: -1, y: 1, z: 0 } => Self::Edge,
            IVec3 { x: 0, y: 1, z: 1 } => Self::Edge,
            IVec3 { x: 0, y: 1, z: -1 } => Self::Edge,
            IVec3 { x: 1, y: 0, z: 1 } => Self::Edge,
            IVec3 { x: 1, y: 0, z: -1 } => Self::Edge,
            IVec3 { x: -1, y: 0, z: 1 } => Self::Edge,
            IVec3 { x: -1, y: 0, z: -1 } => Self::Edge,
            IVec3 { x: 1, y: -1, z: 0 } => Self::Edge,
            IVec3 { x: -1, y: -1, z: 0 } => Self::Edge,
            IVec3 { x: 0, y: -1, z: 1 } => Self::Edge,
            IVec3 { x: 0, y: -1, z: -1 } => Self::Edge,

            IVec3 { x: 1, y: 1, z: 1 } => Self::Corner,
            IVec3 { x: 1, y: 1, z: -1 } => Self::Corner,
            IVec3 { x: -1, y: 1, z: 1 } => Self::Corner,
            IVec3 { x: -1, y: 1, z: -1 } => Self::Corner,
            IVec3 { x: 1, y: -1, z: 1 } => Self::Corner,
            IVec3 { x: 1, y: -1, z: -1 } => Self::Corner,
            IVec3 { x: -1, y: -1, z: 1 } => Self::Corner,
            IVec3 { x: -1, y: -1, z: -1 } => Self::Corner,

            _ => unreachable!("Chunk rotation out of range"),
        }
    }
}
