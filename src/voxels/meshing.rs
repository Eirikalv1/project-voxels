use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

use super::chunk::*;
use super::helper_functions::*;
use crate::utils::*;

type PositionData = Vec<[f32; 3]>;
type NormalData = Vec<[f32; 3]>;
type UvData = Vec<[f32; 2]>;

impl From<Chunk> for Mesh {
    fn from(chunk: Chunk) -> Self {
        let mut quad_poses: PositionData = vec![];
        let mut quad_normals: NormalData = vec![];
        let mut quad_uvs: UvData = vec![];

        for quad in 0..6 {
            for (pos1d, voxel_type) in chunk.voxels.iter().enumerate() {
                if *voxel_type == VoxelType::Block {
                    let pos3d = to_3d(pos1d as f32);
                    if should_create_quad(quad, chunk.voxels, pos3d) {
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
        for quad in 0..(quad_poses.len()) {
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
}

fn should_create_quad(quad: usize, voxels: ChunkData, pos: Vec3) -> bool {
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

fn get_quad_data(
    quad: usize,
    pos: Vec3,
    offset: Vec3,
) -> (PositionData, NormalData, UvData) {
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
