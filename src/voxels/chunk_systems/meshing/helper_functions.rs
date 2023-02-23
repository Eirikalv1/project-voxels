use bevy::prelude::Vec3;

use crate::utils::*;
use crate::voxels::chunk_systems::chunk::*;

pub type PositionData = Vec<[f32; 3]>;
pub type NormalData = Vec<[f32; 3]>;
pub type UvData = Vec<[f32; 2]>;

pub fn voxel_to_right(pos3d: Vec3, voxels: &ChunkData) -> bool {
    pos3d.x + 1. < CHUNK_SIZE && voxels.get(to_1d(pos3d.x + 1., pos3d.y, pos3d.z)) == Some(&VoxelVisibility::Opaque)
}

pub fn voxel_to_left(pos3d: Vec3, voxels: &ChunkData) -> bool {
    pos3d.x > 0. && voxels.get(to_1d(pos3d.x - 1., pos3d.y, pos3d.z)) == Some(&VoxelVisibility::Opaque)
}

pub fn voxel_to_top(pos3d: Vec3, voxels: &ChunkData) -> bool {
    pos3d.y + 1. < CHUNK_SIZE && voxels.get(to_1d(pos3d.x, pos3d.y + 1., pos3d.z)) == Some(&VoxelVisibility::Opaque)
}

pub fn voxel_to_bottom(pos3d: Vec3, voxels: &ChunkData) -> bool {
    pos3d.y > 0. && voxels.get(to_1d(pos3d.x, pos3d.y - 1., pos3d.z)) == Some(&VoxelVisibility::Opaque)
}

pub fn voxel_to_front(pos3d: Vec3, voxels: &ChunkData) -> bool {
    pos3d.z + 1. < CHUNK_SIZE && voxels.get(to_1d(pos3d.x, pos3d.y, pos3d.z + 1.)) == Some(&VoxelVisibility::Opaque)
}

pub fn voxel_to_back(pos3d: Vec3, voxels: &ChunkData) -> bool {
    pos3d.z > 0. && voxels.get(to_1d(pos3d.x, pos3d.y, pos3d.z - 1.)) == Some(&VoxelVisibility::Opaque)
}

pub fn quad_is_visible(quad: usize, voxels: &ChunkData, pos: Vec3) -> bool {
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
    let min_x = pos.x + offset.x * CHUNK_SIZE_MINUS_ONE;
    let min_y = pos.y + offset.y * CHUNK_SIZE_MINUS_ONE;
    let min_z = pos.z + offset.z * CHUNK_SIZE_MINUS_ONE;

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

pub fn get_quad_outside_chunk(quad: usize, pos3d: Vec3) -> bool {
    match quad {
        0 => pos3d.x == CHUNK_SIZE_MINUS_ONE,
        1 => pos3d.x == 0.,
        2 => pos3d.y == CHUNK_SIZE_MINUS_ONE,
        3 => pos3d.y == 0.,
        4 => pos3d.z == CHUNK_SIZE_MINUS_ONE,
        5 => pos3d.z == 0.,
        _ => panic!("Quad indexing out of range"),
    }
}

pub fn adjacent_quad_to_1d(quad: usize, pos3d: Vec3) -> usize {
    match quad {
        0 => to_1d(pos3d.x - CHUNK_SIZE_MINUS_ONE, pos3d.y, pos3d.z),
        1 => to_1d(pos3d.x + CHUNK_SIZE_MINUS_ONE, pos3d.y, pos3d.z),
        2 => to_1d(pos3d.x, pos3d.y - CHUNK_SIZE_MINUS_ONE, pos3d.z),
        3 => to_1d(pos3d.x, pos3d.y + CHUNK_SIZE_MINUS_ONE, pos3d.z),
        4 => to_1d(pos3d.x, pos3d.y, pos3d.z - CHUNK_SIZE_MINUS_ONE),
        5 => to_1d(pos3d.x, pos3d.y, pos3d.z + CHUNK_SIZE_MINUS_ONE),
        _ => panic!("Quad indexing out of range"),
    }
}

pub fn switch_quad_side(quad: usize, pos3d: Vec3) -> (usize, Vec3) {
    match quad {
        0 => (1, Vec3::new(pos3d.x + 1., pos3d.y, pos3d.z)),
        1 => (0, Vec3::new(pos3d.x - 1., pos3d.y, pos3d.z)),
        2 => (3, Vec3::new(pos3d.x, pos3d.y + 1., pos3d.z)),
        3 => (2, Vec3::new(pos3d.x, pos3d.y - 1., pos3d.z)),
        4 => (5, Vec3::new(pos3d.x, pos3d.y, pos3d.z + 1.)),
        5 => (4, Vec3::new(pos3d.x, pos3d.y, pos3d.z - 1.)),
        _ => panic!("Quad indexing out of range"),
    }
}
