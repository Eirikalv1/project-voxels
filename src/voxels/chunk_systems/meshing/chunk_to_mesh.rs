use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

use super::helper_functions::*;
use crate::utils::*;
use crate::voxels::chunk_systems::chunk::*;

pub fn to_mesh(voxels: &ChunkData, chunk_pos: Vec3, adjacent_chunks: [Option<&Chunk>; 6]) -> Mesh {
    let mut quad_poses: PositionData = vec![];
    let mut quad_normals: NormalData = vec![];
    let mut quad_uvs: UvData = vec![];

    for (pos1d, voxel_visibility) in voxels.iter().enumerate() {
        for (mut quad, adjacent_chunk) in adjacent_chunks.iter().enumerate() {
            let mut pos3d = to_3d(pos1d);
            let chunk_pos = chunk_pos;
            let mut should_create_quad = false;

            let quad_outside_chunk = get_quad_outside_chunk(quad, pos3d);

            if quad_is_visible(quad, voxels, pos3d)
                && !quad_outside_chunk
                && *voxel_visibility == VoxelVisibility::Opaque
            {
                should_create_quad = true;
            }

            if adjacent_chunk.is_some() {
                let adjacent_chunk = adjacent_chunk.unwrap();
                let adjacent_quad_outside_chunk = get_quad_outside_chunk(quad, pos3d);

                let adjacent_quad_1d = adjacent_quad_to_1d(quad, pos3d);
                if quad_outside_chunk
                    && adjacent_chunk.voxels[adjacent_quad_1d] == VoxelVisibility::Empty
                    && *voxel_visibility == VoxelVisibility::Opaque
                {
                    should_create_quad = true;
                }
                if adjacent_quad_outside_chunk
                    && voxels[pos1d] == VoxelVisibility::Empty
                    && adjacent_chunk.voxels[adjacent_quad_1d] == VoxelVisibility::Opaque
                {
                    should_create_quad = true;
                    (quad, pos3d) = switch_quad_side(quad, pos3d);
                }
            }
            if should_create_quad {
                let (mut quad_pos, mut quad_normal, mut quad_uv) =
                    get_quad_data(quad, pos3d, chunk_pos);
                quad_poses.append(&mut quad_pos);
                quad_normals.append(&mut quad_normal);
                quad_uvs.append(&mut quad_uv);
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
