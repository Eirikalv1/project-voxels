use bevy::{pbr::wireframe::Wireframe, prelude::*};

use crate::utils::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VoxelType {
    Air,
    Block,
}

#[derive(Clone, Copy)]
pub struct Chunk {
    pub voxels: [VoxelType; CHUNK_VOLUME],
    pub world_pos: Vec3,
}

impl Chunk {
    pub fn new(voxels: [VoxelType; CHUNK_VOLUME], world_pos: Vec3) -> Self {
        Self { voxels, world_pos }
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
        chunk: Chunk,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        Self {
            pbr_bundle: PbrBundle {
                mesh: meshes.add(chunk.into()),
                material: materials.add(Color::rgb(0.2, 0.2, 0.7).into()),
                transform: Transform {
                    translation: chunk.world_pos,
                    ..default()
                },
                ..default()
            },
            wireframe: Wireframe,
            name: Name::new(format!("Chunk [{}]", chunk.world_pos)),
        }
    }
}
