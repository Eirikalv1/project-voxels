use bevy::prelude::Vec3;

use crate::utils::*;
use crate::voxels::chunk_systems::chunk::*;

pub type PositionData = Vec<[f32; 3]>;
pub type NormalData = Vec<[f32; 3]>;
pub type UVData = Vec<[f32; 2]>;
pub type AOData = Vec<[f32; 4]>;
pub type IndicesData = Vec<u32>;

pub fn quad_is_visible(quad: usize, voxels: &ChunkData, pos: Vec3) -> bool {
    match quad {
        0 => {
            pos.x + 1. < CHUNK_SIZE
                && voxels.get(Chunk::linearize(pos.x + 1., pos.y, pos.z)) == Some(&VoxelVisibility::Empty)
        }
        1 => pos.x > 0. && voxels.get(Chunk::linearize(pos.x - 1., pos.y, pos.z)) == Some(&VoxelVisibility::Empty),
        2 => {
            pos.y + 1. < CHUNK_SIZE
                && voxels.get(Chunk::linearize(pos.x, pos.y + 1., pos.z)) == Some(&VoxelVisibility::Empty)
        }
        3 => pos.y > 0. && voxels.get(Chunk::linearize(pos.x, pos.y - 1., pos.z)) == Some(&VoxelVisibility::Empty),
        4 => {
            pos.z + 1. < CHUNK_SIZE
                && voxels.get(Chunk::linearize(pos.x, pos.y, pos.z + 1.)) == Some(&VoxelVisibility::Empty)
        }
        5 => pos.z > 0. && voxels.get(Chunk::linearize(pos.x, pos.y, pos.z - 1.)) == Some(&VoxelVisibility::Empty),
        _ => false,
    }
}

pub fn get_quad_data(quad: usize, pos: Vec3, offset: Vec3) -> (PositionData, NormalData, UVData) {
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
        _ => unreachable!("Quad indexing out of range"),
    }
}

pub fn adjacent_quad_linearize(quad: usize, pos3d: Vec3) -> usize {
    match quad {
        0 => Chunk::linearize(pos3d.x - CHUNK_SIZE_MINUS_ONE, pos3d.y, pos3d.z),
        1 => Chunk::linearize(pos3d.x + CHUNK_SIZE_MINUS_ONE, pos3d.y, pos3d.z),
        2 => Chunk::linearize(pos3d.x, pos3d.y - CHUNK_SIZE_MINUS_ONE, pos3d.z),
        3 => Chunk::linearize(pos3d.x, pos3d.y + CHUNK_SIZE_MINUS_ONE, pos3d.z),
        4 => Chunk::linearize(pos3d.x, pos3d.y, pos3d.z - CHUNK_SIZE_MINUS_ONE),
        5 => Chunk::linearize(pos3d.x, pos3d.y, pos3d.z + CHUNK_SIZE_MINUS_ONE),
        _ => unreachable!("Quad indexing out of range"),
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
        _ => unreachable!("Quad indexing out of range"),
    }
}
