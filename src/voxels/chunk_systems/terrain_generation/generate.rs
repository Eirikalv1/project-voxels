use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};

use crate::utils::*;
use crate::voxels::chunk_systems::chunk::*;

use super::chunk_rotation::{
    chunk_inside_world_radius, chunk_outside_world_radius, get_chunk_rotation, rotate_chunk, should_rotate_chunk,
};

const AMPLITUDE: f64 = 20.;
const FREQUENCY: f64 = 10.;

pub fn gen_terrain(chunk_pos: IVec3) -> ChunkData {
    let mut chunk_data: ChunkData = Box::new([VoxelVisibility::Empty; CHUNK_VOLUME]);

    let chunk_rot = get_chunk_rotation(chunk_pos);

    if chunk_outside_world_radius(chunk_pos) {
        return chunk_data;
    }

    if chunk_inside_world_radius(chunk_rot) {
        return Box::new([VoxelVisibility::Opaque; CHUNK_VOLUME]);
    }

    if !should_rotate_chunk(chunk_rot) {
        return chunk_data;
    }

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
