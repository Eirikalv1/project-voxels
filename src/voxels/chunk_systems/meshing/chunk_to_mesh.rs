use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

use super::ambient_occlusion::{get_ao_data, get_neighbouring_voxels, should_flip_quad};
use super::helper_functions::*;
use crate::utils::*;
use crate::voxels::chunk_systems::chunk::*;

const INDICES_NOT_FLIPPED: [u32; 6] = [0, 1, 2, 2, 3, 0];
const INDICES_FLIPPED: [u32; 6] = [3, 0, 1, 1, 2, 3];

pub fn to_mesh(voxels: &ChunkData, chunk_pos: Vec3, adjacent_chunks: [Option<&Chunk>; 6]) -> Mesh {
    let mut quad_poses: PositionData = vec![];
    let mut quad_normals: NormalData = vec![];
    let mut quad_uvs: UVData = vec![];
    let mut quad_aos: AOData = vec![];
    let mut quad_indices: IndicesData = vec![];

    let mut indicies_index: u32 = 0;

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
                let (mut quad_pos, mut quad_normal, mut quad_uv) = get_quad_data(quad, pos3d, chunk_pos);
                quad_poses.append(&mut quad_pos);
                quad_normals.append(&mut quad_normal);
                quad_uvs.append(&mut quad_uv);

                let mut ao_data = get_ao_data(get_neighbouring_voxels(quad, pos3d, voxels));

                if should_flip_quad(&ao_data) {
                    quad_indices.append(&mut vec![
                        INDICES_FLIPPED[0] + 4 * indicies_index,
                        INDICES_FLIPPED[1] + 4 * indicies_index,
                        INDICES_FLIPPED[2] + 4 * indicies_index,
                        INDICES_FLIPPED[3] + 4 * indicies_index,
                        INDICES_FLIPPED[4] + 4 * indicies_index,
                        INDICES_FLIPPED[5] + 4 * indicies_index,
                    ]);
                } else {
                    quad_indices.append(&mut vec![
                        INDICES_NOT_FLIPPED[0] + 4 * indicies_index,
                        INDICES_NOT_FLIPPED[1] + 4 * indicies_index,
                        INDICES_NOT_FLIPPED[2] + 4 * indicies_index,
                        INDICES_NOT_FLIPPED[3] + 4 * indicies_index,
                        INDICES_NOT_FLIPPED[4] + 4 * indicies_index,
                        INDICES_NOT_FLIPPED[5] + 4 * indicies_index,
                    ]);
                }

                quad_aos.append(&mut ao_data);

                indicies_index += 1;
            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, quad_poses);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, quad_normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, quad_uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, quad_aos);
    mesh.set_indices(Some(Indices::U32(quad_indices)));
    mesh
}
