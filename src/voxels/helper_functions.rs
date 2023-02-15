use bevy::prelude::Vec3;

use crate::utils::*;

pub fn voxel_to_right(pos3d: Vec3, voxels: [VoxelType; CHUNK_VOLUME]) -> bool {
    pos3d.x + 1. < CHUNK_SIZE
        && voxels.get(to_1d(pos3d.x + 1., pos3d.y, pos3d.z) as usize) == Some(&VoxelType::Block)
}

pub fn voxel_to_left(pos3d: Vec3, voxels: [VoxelType; CHUNK_VOLUME]) -> bool {
    pos3d.x > 0.
        && voxels.get(to_1d(pos3d.x - 1., pos3d.y, pos3d.z) as usize) == Some(&VoxelType::Block)
}

pub fn voxel_to_top(pos3d: Vec3, voxels: [VoxelType; CHUNK_VOLUME]) -> bool {
    pos3d.y + 1. < CHUNK_SIZE
        && voxels.get(to_1d(pos3d.x, pos3d.y + 1., pos3d.z) as usize) == Some(&VoxelType::Block)
}

pub fn voxel_to_bottom(pos3d: Vec3, voxels: [VoxelType; CHUNK_VOLUME]) -> bool {
    pos3d.y > 0.
        && voxels.get(to_1d(pos3d.x, pos3d.y - 1., pos3d.z) as usize) == Some(&VoxelType::Block)
}

pub fn voxel_to_front(pos3d: Vec3, voxels: [VoxelType; CHUNK_VOLUME]) -> bool {
    pos3d.z + 1. < CHUNK_SIZE
        && voxels.get(to_1d(pos3d.x, pos3d.y, pos3d.z + 1.) as usize) == Some(&VoxelType::Block)
}

pub fn voxel_to_back(pos3d: Vec3, voxels: [VoxelType; CHUNK_VOLUME]) -> bool {
    pos3d.z > 0.
        && voxels.get(to_1d(pos3d.x, pos3d.y, pos3d.z - 1.) as usize) == Some(&VoxelType::Block)
}
