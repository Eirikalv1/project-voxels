use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};

use crate::utils::*;
use crate::voxels::chunk_systems::chunk::*;

use super::chunk_rotation::{get_chunk_rotation, rotate_chunk, chunk_pos_flattend_horistonally};
use super::chunk_type::{get_corner_direction, get_edge_direction, ChunkType};

const AMPLITUDE: f64 = 20.;
const FREQUENCY: f64 = 10.;

pub fn gen_terrain(chunk_pos: IVec3) -> ChunkData {
    let chunk_rot = get_chunk_rotation(chunk_pos);

    match ChunkType::get_chunk_type(chunk_pos, chunk_rot) {
        ChunkType::Inside => return Box::new([VoxelVisibility::Opaque; CHUNK_VOLUME]),
        ChunkType::Outside => return Box::new([VoxelVisibility::Empty; CHUNK_VOLUME]),
        ChunkType::Corner => {
            let (dir_a, dir_b, dir_c) = get_corner_direction(chunk_rot);
            let (chunk_data_a, chunk_data_b, chunk_data_c) = (
                gen_with_noise(chunk_pos, dir_a),
                gen_with_noise(chunk_pos, dir_b),
                gen_with_noise(chunk_pos, dir_c),
            );
            let mut corner_chunk_data = Box::new([VoxelVisibility::Empty; CHUNK_VOLUME]);

            for pos in 0..chunk_data_a.len() {
                if chunk_data_a[pos] == VoxelVisibility::Opaque
                    && chunk_data_b[pos] == VoxelVisibility::Opaque
                    && chunk_data_c[pos] == VoxelVisibility::Opaque
                {
                    corner_chunk_data[pos] = VoxelVisibility::Opaque;
                }
            }

            return corner_chunk_data;
        }
        ChunkType::Edge => {
            let (dir_a, dir_b) = get_edge_direction(chunk_rot);
            let (chunk_data_a, chunk_data_b) = (gen_with_noise(chunk_pos, dir_a), gen_with_noise(chunk_pos, dir_b));
            let mut edge_chunk_data = Box::new([VoxelVisibility::Empty; CHUNK_VOLUME]);

            for pos in 0..chunk_data_a.len() {
                if chunk_data_a[pos] == VoxelVisibility::Opaque && chunk_data_b[pos] == VoxelVisibility::Opaque {
                    edge_chunk_data[pos] = VoxelVisibility::Opaque;
                }
            }

            return edge_chunk_data;
        }
        ChunkType::Center => return gen_with_noise(chunk_pos, chunk_rot),
    }
}

fn gen_with_noise(mut chunk_pos: IVec3, chunk_rot: IVec3) -> ChunkData {
    let mut chunk_data: ChunkData = Box::new([VoxelVisibility::Empty; CHUNK_VOLUME]);
    let noise = OpenSimplex::new(0);

    chunk_pos = chunk_pos_flattend_horistonally(chunk_pos, chunk_rot);

    for pos1d in 0..CHUNK_VOLUME {
        let pos3d = to_3d(pos1d);
        let world_pos3d = to_world_pos(pos3d, chunk_pos);

        let height = (noise.get([world_pos3d.x as f64 / FREQUENCY, 0.0, world_pos3d.z as f64 / FREQUENCY]) * AMPLITUDE
            + AMPLITUDE / 2.0) as i32;

        for y in 0..height {
            let rotated_chunk = rotate_chunk(chunk_rot, Vec3::new(pos3d.x, y as f32, pos3d.z));

            if chunk_data
                .get(to_1d(rotated_chunk.x, rotated_chunk.y, rotated_chunk.z))
                .is_some()
            {
                chunk_data[to_1d(rotated_chunk.x, rotated_chunk.y, rotated_chunk.z)] = VoxelVisibility::Opaque;
            }
        }
    }
    chunk_data
}
