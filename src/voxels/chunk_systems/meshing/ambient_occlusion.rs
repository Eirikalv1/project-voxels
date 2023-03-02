use bevy::prelude::Vec3;

use crate::voxels::chunk_systems::chunk::{Chunk, ChunkData, VoxelVisibility};

use super::quad::AOData;

pub type VoxelNeighbours = [VoxelVisibility; 8];

fn get_neighbouring_voxel(current_voxel: Vec3, voxel_to_check: Vec3, voxels: &ChunkData) -> VoxelVisibility {
    if let Some(voxel) = voxels.get(Chunk::linearize(
        current_voxel.x + voxel_to_check.x,
        current_voxel.y + voxel_to_check.y,
        current_voxel.z + voxel_to_check.z,
    )) {
        return *voxel;
    }
    VoxelVisibility::Empty
}

pub fn get_neighbouring_voxels(quad: usize, pos: Vec3, voxels: &ChunkData) -> VoxelNeighbours {
    let mut neighbours = [VoxelVisibility::Empty; 8];
    match quad {
        0 => {
            // PosX
            neighbours[0] = get_neighbouring_voxel(pos, Vec3::new(1., 0., -1.), voxels);
            neighbours[1] = get_neighbouring_voxel(pos, Vec3::new(1., -1., -1.), voxels);
            neighbours[2] = get_neighbouring_voxel(pos, Vec3::new(1., -1., 0.), voxels);
            neighbours[3] = get_neighbouring_voxel(pos, Vec3::new(1., -1., 1.), voxels);
            neighbours[4] = get_neighbouring_voxel(pos, Vec3::new(1., 0., 1.), voxels);
            neighbours[5] = get_neighbouring_voxel(pos, Vec3::new(1., 1., 1.), voxels);
            neighbours[6] = get_neighbouring_voxel(pos, Vec3::new(1., 1., 0.), voxels);
            neighbours[7] = get_neighbouring_voxel(pos, Vec3::new(1., 1., -1.), voxels);
        }
        1 => {
            // NegX
            neighbours[0] = get_neighbouring_voxel(pos, Vec3::new(-1., 0., 1.), voxels);
            neighbours[1] = get_neighbouring_voxel(pos, Vec3::new(-1., -1., 1.), voxels);
            neighbours[2] = get_neighbouring_voxel(pos, Vec3::new(-1., -1., 0.), voxels);
            neighbours[3] = get_neighbouring_voxel(pos, Vec3::new(-1., -1., -1.), voxels);
            neighbours[4] = get_neighbouring_voxel(pos, Vec3::new(-1., 0., -1.), voxels);
            neighbours[5] = get_neighbouring_voxel(pos, Vec3::new(-1., 1., -1.), voxels);
            neighbours[6] = get_neighbouring_voxel(pos, Vec3::new(-1., 1., 0.), voxels);
            neighbours[7] = get_neighbouring_voxel(pos, Vec3::new(-1., 1., 1.), voxels);
        }

        2 => {
            // PosY
            neighbours[0] = get_neighbouring_voxel(pos, Vec3::new(0., 1., -1.), voxels);
            neighbours[1] = get_neighbouring_voxel(pos, Vec3::new(1., 1., -1.), voxels);
            neighbours[2] = get_neighbouring_voxel(pos, Vec3::new(1., 1., 0.), voxels);
            neighbours[3] = get_neighbouring_voxel(pos, Vec3::new(1., 1., 1.), voxels);
            neighbours[4] = get_neighbouring_voxel(pos, Vec3::new(0., 1., 1.), voxels);
            neighbours[5] = get_neighbouring_voxel(pos, Vec3::new(-1., 1., 1.), voxels);
            neighbours[6] = get_neighbouring_voxel(pos, Vec3::new(-1., 1., 0.), voxels);
            neighbours[7] = get_neighbouring_voxel(pos, Vec3::new(-1., 1., -1.), voxels);
        }
        3 => {
            // NegY
            neighbours[0] = get_neighbouring_voxel(pos, Vec3::new(0., -1., 1.), voxels);
            neighbours[1] = get_neighbouring_voxel(pos, Vec3::new(1., -1., 1.), voxels);
            neighbours[2] = get_neighbouring_voxel(pos, Vec3::new(1., -1., 0.), voxels);
            neighbours[3] = get_neighbouring_voxel(pos, Vec3::new(1., -1., -1.), voxels);
            neighbours[4] = get_neighbouring_voxel(pos, Vec3::new(0., -1., -1.), voxels);
            neighbours[5] = get_neighbouring_voxel(pos, Vec3::new(-1., -1., -1.), voxels);
            neighbours[6] = get_neighbouring_voxel(pos, Vec3::new(-1., -1., 0.), voxels);
            neighbours[7] = get_neighbouring_voxel(pos, Vec3::new(-1., -1., 1.), voxels);
        }
        4 => {
            // PosZ
            neighbours[0] = get_neighbouring_voxel(pos, Vec3::new(0., -1., 1.), voxels);
            neighbours[1] = get_neighbouring_voxel(pos, Vec3::new(-1., -1., 1.), voxels);
            neighbours[2] = get_neighbouring_voxel(pos, Vec3::new(-1., 0., 1.), voxels);
            neighbours[3] = get_neighbouring_voxel(pos, Vec3::new(-1., 1., 1.), voxels);
            neighbours[4] = get_neighbouring_voxel(pos, Vec3::new(0., 1., 1.), voxels);
            neighbours[5] = get_neighbouring_voxel(pos, Vec3::new(1., 1., 1.), voxels);
            neighbours[6] = get_neighbouring_voxel(pos, Vec3::new(1., 0., 1.), voxels);
            neighbours[7] = get_neighbouring_voxel(pos, Vec3::new(1., -1., 1.), voxels);
        }
        5 => {
            // NegZ
            neighbours[0] = get_neighbouring_voxel(pos, Vec3::new(0., 1., -1.), voxels);
            neighbours[1] = get_neighbouring_voxel(pos, Vec3::new(-1., 1., -1.), voxels);
            neighbours[2] = get_neighbouring_voxel(pos, Vec3::new(-1., 0., -1.), voxels);
            neighbours[3] = get_neighbouring_voxel(pos, Vec3::new(-1., -1., -1.), voxels);
            neighbours[4] = get_neighbouring_voxel(pos, Vec3::new(0., -1., -1.), voxels);
            neighbours[5] = get_neighbouring_voxel(pos, Vec3::new(1., -1., -1.), voxels);
            neighbours[6] = get_neighbouring_voxel(pos, Vec3::new(1., 0., -1.), voxels);
            neighbours[7] = get_neighbouring_voxel(pos, Vec3::new(1., 1., -1.), voxels);
        }
        _ => unreachable!("Quad indexing out of range"),
    }

    neighbours
}

const DARK: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
const LIGHT: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

const OPAQUE: VoxelVisibility = VoxelVisibility::Opaque;

pub fn get_ao_data(neighbours: VoxelNeighbours) -> AOData {
    let mut ao_data: AOData = vec![LIGHT, LIGHT, LIGHT, LIGHT];

    ao_data[0] = get_vertex_color([neighbours[0], neighbours[1], neighbours[2]]);
    ao_data[3] = get_vertex_color([neighbours[2], neighbours[3], neighbours[4]]);
    ao_data[2] = get_vertex_color([neighbours[4], neighbours[5], neighbours[6]]);
    ao_data[1] = get_vertex_color([neighbours[6], neighbours[7], neighbours[0]]);

    ao_data
}

fn get_vertex_color(neighbours: [VoxelVisibility; 3]) -> [f32; 4] {
    match neighbours {
        [OPAQUE, OPAQUE, OPAQUE]
        | [OPAQUE, _, OPAQUE]
        | [OPAQUE, OPAQUE, _]
        | [_, OPAQUE, OPAQUE]
        | [OPAQUE, _, _]
        | [_, OPAQUE, _]
        | [_, _, OPAQUE] => DARK,
        _ => LIGHT,
    }
}

pub fn should_flip_quad(ao_data: &AOData) -> bool {
    ao_data[0][0] + ao_data[2][0] < ao_data[1][0] + ao_data[3][0]
}
