use bevy::prelude::Vec3;

use crate::utils::*;

pub type PositionData = Vec<[f32; 3]>;
pub type NormalData = Vec<[f32; 3]>;
pub type UvData = Vec<[f32; 2]>;

pub fn voxel_to_right(pos3d: Vec3, voxels: &ChunkData) -> bool {
    pos3d.x + 1. < CHUNK_SIZE
        && voxels.get(to_1d(pos3d.x + 1., pos3d.y, pos3d.z)) == Some(&VoxelType::Block)
}

pub fn voxel_to_left(pos3d: Vec3, voxels: &ChunkData) -> bool {
    pos3d.x > 0. && voxels.get(to_1d(pos3d.x - 1., pos3d.y, pos3d.z)) == Some(&VoxelType::Block)
}

pub fn voxel_to_top(pos3d: Vec3, voxels: &ChunkData) -> bool {
    pos3d.y + 1. < CHUNK_SIZE
        && voxels.get(to_1d(pos3d.x, pos3d.y + 1., pos3d.z)) == Some(&VoxelType::Block)
}

pub fn voxel_to_bottom(pos3d: Vec3, voxels: &ChunkData) -> bool {
    pos3d.y > 0. && voxels.get(to_1d(pos3d.x, pos3d.y - 1., pos3d.z)) == Some(&VoxelType::Block)
}

pub fn voxel_to_front(pos3d: Vec3, voxels: &ChunkData) -> bool {
    pos3d.z + 1. < CHUNK_SIZE
        && voxels.get(to_1d(pos3d.x, pos3d.y, pos3d.z + 1.)) == Some(&VoxelType::Block)
}

pub fn voxel_to_back(pos3d: Vec3, voxels: &ChunkData) -> bool {
    pos3d.z > 0. && voxels.get(to_1d(pos3d.x, pos3d.y, pos3d.z - 1.)) == Some(&VoxelType::Block)
}

pub fn should_create_quad(quad: usize, voxels: &ChunkData, pos: Vec3) -> bool {
    match quad {
        0 => !voxel_to_right(pos, voxels),
        1 => !voxel_to_left(pos, voxels),
        2 => !voxel_to_top(pos, voxels),
        3 => !voxel_to_bottom(pos, voxels),
        4 => !voxel_to_front(pos, voxels),
        5 => !voxel_to_back(pos, voxels),
        _ => false,
    }
}

pub fn get_quad_data(quad: usize, pos: Vec3, offset: Vec3) -> (PositionData, NormalData, UvData) {
    let min_x = pos.x + offset.x * (CHUNK_SIZE - 1.);
    let min_y = pos.y + offset.y * (CHUNK_SIZE - 1.);
    let min_z = pos.z + offset.z * (CHUNK_SIZE - 1.);

    let max_x = min_x + 1.;
    let max_y = min_y + 1.;
    let max_z = min_z + 1.;

    match quad {
        0 => (
            vec![
                [max_x, min_y, min_z],
                [max_x, max_y, min_z],
                [max_x, max_y, max_z],
                [max_x, min_y, max_z],
            ],
            vec![[1., 0., 0.]; 4],
            vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]],
        ),
        1 => (
            vec![
                [min_x, min_y, max_z],
                [min_x, max_y, max_z],
                [min_x, max_y, min_z],
                [min_x, min_y, min_z],
            ],
            vec![[-1., 0., 0.]; 4],
            vec![[1., 0.], [0., 0.], [0., 1.], [1., 1.]],
        ),
        2 => (
            vec![
                [max_x, max_y, min_z],
                [min_x, max_y, min_z],
                [min_x, max_y, max_z],
                [max_x, max_y, max_z],
            ],
            vec![[0., 1., 0.]; 4],
            vec![[1., 0.], [0., 0.], [0., 1.], [1., 1.]],
        ),
        3 => (
            vec![
                [max_x, min_y, max_z],
                [min_x, min_y, max_z],
                [min_x, min_y, min_z],
                [max_x, min_y, min_z],
            ],
            vec![[0., -1., 0.]; 4],
            vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]],
        ),
        4 => (
            vec![
                [min_x, min_y, max_z],
                [max_x, min_y, max_z],
                [max_x, max_y, max_z],
                [min_x, max_y, max_z],
            ],
            vec![[0., 0., 1.]; 4],
            vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]],
        ),
        5 => (
            vec![
                [min_x, max_y, min_z],
                [max_x, max_y, min_z],
                [max_x, min_y, min_z],
                [min_x, min_y, min_z],
            ],
            vec![[0., 0., -1.]; 4],
            vec![[1., 0.], [0., 0.], [0., 1.], [1., 1.]],
        ),
        _ => panic!("Quad indexing out of range"),
    }
}