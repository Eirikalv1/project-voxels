use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};

use super::chunk::*;
use crate::utils::*;

const AMPLITUDE: f64 = 20.;
const FREQUENCY: f64 = 10.;

pub fn gen_terrain(chunk_pos: IVec3) -> ChunkData {
    let mut chunk_data: ChunkData = Box::new([VoxelVisibility::Empty; CHUNK_VOLUME]);

    let noise = OpenSimplex::new(0);

    if chunk_pos.y > 0 {
        return chunk_data;
    }
    if chunk_pos.y < 0 {
        return Box::new([VoxelVisibility::Opaque; CHUNK_VOLUME]);
    }

    for pos1d in 0..CHUNK_VOLUME {
        let pos3d = to_3d(pos1d);
        let world_pos3d = to_world_pos(pos3d, chunk_pos);

        let height = (noise.get([
            world_pos3d.x as f64 / FREQUENCY,
            0.0,
            world_pos3d.z as f64 / FREQUENCY,
        ]) * AMPLITUDE
            + AMPLITUDE / 2.0) as i32;

        for y in 0..height {
            if chunk_data.get(to_1d(pos3d.x, y as f32, pos3d.z)).is_some() {
                chunk_data[to_1d(pos3d.x, y as f32, pos3d.z)] = VoxelVisibility::Opaque;
            }
        }
    }
    chunk_data
}
