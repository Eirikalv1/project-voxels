use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};

use super::chunk::*;
use crate::utils::*;

const AMPLITUDE: f64 = 15.;
const FREQUENCY: f64 = 10.;

pub fn gen_terrain(world_pos: Vec3) -> [VoxelType; CHUNK_SIZE_CUBED] {
    let mut chunk_data: [VoxelType; CHUNK_SIZE_CUBED] = [VoxelType::Air; CHUNK_SIZE_CUBED];

    let noise = OpenSimplex::new(0);

    for pos1d in 0..CHUNK_SIZE_CUBED {
        let pos3d = to_3d(pos1d as f32);
        let world_pos3d = pos3d + world_pos * CHUNK_SIZE;

        let height = (noise.get([
            world_pos3d.x as f64 / FREQUENCY,
            0.0,
            world_pos3d.z as f64 / FREQUENCY,
        ]) * AMPLITUDE
            + AMPLITUDE / 2.0) as i32;

        for y in 0..height {
            chunk_data[to_1d(pos3d.x as f32, y as f32, pos3d.z as f32) as usize] = VoxelType::Block;
        }
    }
    chunk_data
}