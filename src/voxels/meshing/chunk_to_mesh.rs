use std::collections::HashMap;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

use super::helper_functions::*;
use crate::utils::*;
use crate::voxels::chunk::*;

pub fn to_mesh(chunk: &Chunk, chunks: &HashMap<(usize, usize, usize), Chunk>) -> Mesh {
    let mut quad_poses: PositionData = vec![];
    let mut quad_normals: NormalData = vec![];
    let mut quad_uvs: UvData = vec![];

    for (pos1d, voxel_type) in chunk.voxels.iter().enumerate() {
        if *voxel_type == VoxelType::Block {
            for quad in 0..6 {
                let pos3d = to_3d(pos1d as f32);
                if should_create_quad(quad, &chunk.voxels, pos3d)
                //&& !quad_outside_chunk(quad, pos3d)
                {
                    let (mut quad_pos, mut quad_normal, mut quad_uv) =
                        get_quad_data(quad, pos3d, chunk.world_pos);
                    quad_poses.append(&mut quad_pos);
                    quad_normals.append(&mut quad_normal);
                    quad_uvs.append(&mut quad_uv);
                }
            }
        }
    }

    let mut indices: Vec<u32> = vec![];
    for quad in 0..(quad_poses.len() / 4) {
        indices.append(&mut vec![
            4 * quad as u32,
            1 + 4 * quad as u32,
            2 + 4 * quad as u32,
            2 + 4 * quad as u32,
            3 + 4 * quad as u32,
            4 * quad as u32,
        ]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, quad_poses);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, quad_normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, quad_uvs);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

fn get_adjacent_chunk(pos3d: Vec3, chunks: HashMap<(usize, usize, usize), &Chunk>) -> [Chunk; 6] {
    let pos: (usize, usize, usize) = (pos3d.x as usize, pos3d.y as usize, pos3d.z as usize);

    [
        chunks[&(pos.0 + 1, pos.1, pos.2)].clone(),
        chunks[&(pos.0 - 1, pos.1, pos.2)].clone(),
        chunks[&(pos.0, pos.1 + 1, pos.2)].clone(),
        chunks[&(pos.0, pos.1 - 1, pos.2)].clone(),
        chunks[&(pos.0, pos.1, pos.2 + 1)].clone(),
        chunks[&(pos.0, pos.1, pos.2 - 1)].clone(),
    ]
}

/*
fn quad_outside_chunk(quad: usize, pos3d: Vec3) -> bool {
    match quad {
        0 => pos3d.x == CHUNK_SIZE - 1.,
        1 => pos3d.x == 0.,
        2 => pos3d.y == CHUNK_SIZE - 1.,
        3 => pos3d.y == 0.,
        4 => pos3d.z == CHUNK_SIZE - 1.,
        5 => pos3d.z == 0.,
        _ => false,
    }
}
*/
