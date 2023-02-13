use bevy::prelude::*;

use crate::utils::CHUNK_SIZE_CUBED;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VoxelType {
    Air,
    Block,
}

#[derive(Clone, Copy)]
pub struct Chunk {
    pub voxels: [VoxelType; CHUNK_SIZE_CUBED],
    pub world_pos: Vec3,
}

impl Chunk {
    pub fn new(voxels: [VoxelType; CHUNK_SIZE_CUBED], world_pos: Vec3) -> Self {
        Self { voxels, world_pos }
    }
}
