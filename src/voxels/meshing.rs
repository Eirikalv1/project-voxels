use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

use super::chunk::*;
use super::helper_functions::*;
use crate::utils::*;

type VertexData = (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>);

impl From<Chunk> for Mesh {
    fn from(chunk: Chunk) -> Self {
        let mut quad_poses: Vec<[f32; 3]> = vec![];
        let mut quad_normals: Vec<[f32; 3]> = vec![];
        let mut quad_uvs: Vec<[f32; 2]> = vec![];

        for (pos1d, voxel_type) in chunk.voxels.iter().enumerate() {
            if *voxel_type == VoxelType::Block {
                let pos3d = to_3d(pos1d as f32);
                let (mut voxel_quad_poses, mut voxel_quad_normals, mut voxel_quad_uvs) =
                    create_voxel(chunk.voxels, pos3d, chunk.world_pos);

                quad_poses.append(&mut voxel_quad_poses);
                quad_normals.append(&mut voxel_quad_normals);
                quad_uvs.append(&mut voxel_quad_uvs);
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

fn create_voxel(voxels: [VoxelType; CHUNK_SIZE_CUBED], pos: Vec3, offset: Vec3) -> VertexData {
    let mut quad_poses: Vec<[f32; 3]> = vec![];
    let mut quad_normals: Vec<[f32; 3]> = vec![];
    let mut quad_uvs: Vec<[f32; 2]> = vec![];

    let min_x = pos.x + offset.x * (CHUNK_SIZE - 1.);
    let min_y = pos.y + offset.y * (CHUNK_SIZE - 1.);
    let min_z = pos.z + offset.z * (CHUNK_SIZE - 1.);

    let max_x = min_x + 1.;
    let max_y = min_y + 1.;
    let max_z = min_z + 1.;

    if !(voxel_to_right(pos, voxels)) {
        quad_poses.append(&mut vec![
            [max_x, min_y, min_z],
            [max_x, max_y, min_z],
            [max_x, max_y, max_z],
            [max_x, min_y, max_z],
        ]);
        quad_normals.append(&mut vec![[1., 0., 0.]; 4]);
        quad_uvs.append(&mut vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]]);
    }

    if !(voxel_to_left(pos, voxels)) {
        quad_poses.append(&mut vec![
            [min_x, min_y, max_z],
            [min_x, max_y, max_z],
            [min_x, max_y, min_z],
            [min_x, min_y, min_z],
        ]);
        quad_normals.append(&mut vec![[-1., 0., 0.]; 4]);
        quad_uvs.append(&mut vec![[1., 0.], [0., 0.], [0., 1.], [1., 1.]]);
    }

    if !(voxel_to_top(pos, voxels)) {
        quad_poses.append(&mut vec![
            [max_x, max_y, min_z],
            [min_x, max_y, min_z],
            [min_x, max_y, max_z],
            [max_x, max_y, max_z],
        ]);
        quad_normals.append(&mut vec![[0., 1., 0.]; 4]);
        quad_uvs.append(&mut vec![[1., 0.], [0., 0.], [0., 1.], [1., 1.]]);
    }

    if !(voxel_to_bottom(pos, voxels)) {
        quad_poses.append(&mut vec![
            [max_x, min_y, max_z],
            [min_x, min_y, max_z],
            [min_x, min_y, min_z],
            [max_x, min_y, min_z],
        ]);
        quad_normals.append(&mut vec![[0., -1., 0.]; 4]);
        quad_uvs.append(&mut vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]]);
    }

    if !(voxel_to_front(pos, voxels)) {
        quad_poses.append(&mut vec![
            [min_x, min_y, max_z],
            [max_x, min_y, max_z],
            [max_x, max_y, max_z],
            [min_x, max_y, max_z],
        ]);
        quad_normals.append(&mut vec![[0., 0., 1.]; 4]);
        quad_uvs.append(&mut vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]]);
    }

    if !(voxel_to_back(pos, voxels)) {
        quad_poses.append(&mut vec![
            [min_x, max_y, min_z],
            [max_x, max_y, min_z],
            [max_x, min_y, min_z],
            [min_x, min_y, min_z],
        ]);
        quad_normals.append(&mut vec![[0., 0., -1.]; 4]);
        quad_uvs.append(&mut vec![[1., 0.], [0., 0.], [0., 1.], [1., 1.]]);
    }

    (quad_poses, quad_normals, quad_uvs)
}
