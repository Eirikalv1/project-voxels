use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};

use crate::utils::*;
use crate::voxels::chunk_systems::chunk::*;

use super::chunk_rotation::{get_chunk_rotation, rotate_chunk};
use super::chunk_type::ChunkType;

const AMPLITUDE: f64 = 20.;
const FREQUENCY: f64 = 10.;

pub fn gen_terrain(chunk_pos: IVec3) -> ChunkData {
    let chunk_rot = get_chunk_rotation(chunk_pos);

    let chunk_type = ChunkType::get_chunk_type(chunk_pos, chunk_rot);
    if chunk_type == ChunkType::Inside {
        return Box::new([VoxelVisibility::Opaque; CHUNK_VOLUME]);
    }

    if chunk_type != ChunkType::Center {
        return Box::new([VoxelVisibility::Empty; CHUNK_VOLUME]);
    }

    gen_with_noise(chunk_pos, chunk_rot)
}

fn gen_with_noise(chunk_pos: IVec3, chunk_rot: IVec3) -> ChunkData {
    let mut chunk_data: ChunkData = Box::new([VoxelVisibility::Empty; CHUNK_VOLUME]);
    let noise = OpenSimplex::new(0);

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
