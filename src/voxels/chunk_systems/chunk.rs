use bevy::{pbr::wireframe::Wireframe, prelude::*};

use super::meshing::chunk_to_mesh::to_mesh;
use crate::utils::*;

pub type ChunkData = Box<[VoxelVisibility; CHUNK_VOLUME]>;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VoxelVisibility {
    Empty,
    Transparent,
    Opaque,
}

#[derive(Clone)]
pub struct Chunk {
    pub voxels: ChunkData,
    pub chunk_pos: Vec3,
}

impl Chunk {
    pub fn new(voxels: ChunkData, chunk_pos: Vec3) -> Self {
        Self { voxels, chunk_pos }
    }
}

#[derive(Bundle)]
pub struct ChunkBundle {
    pub pbr_bundle: PbrBundle,
    pub wireframe: Wireframe,
    pub name: Name,
}

impl ChunkBundle {
    pub fn new(
        chunk: &Chunk,
        adjacent_chunks: [Option<&Chunk>; 6],
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        Self {
            pbr_bundle: PbrBundle {
                mesh: meshes.add(to_mesh(chunk, adjacent_chunks)),
                material: materials.add(Color::rgb(0.2, 0.2, 0.7).into()),
                transform: Transform {
                    translation: chunk.chunk_pos,
                    ..default()
                },
                ..default()
            },
            wireframe: Wireframe,
            name: Name::new(format!("Chunk [{}]", chunk.chunk_pos)),
        }
    }
}
