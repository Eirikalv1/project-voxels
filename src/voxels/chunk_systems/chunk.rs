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

pub struct Chunk {
    pub voxels: ChunkData,
    pub chunk_pos: Vec3,
    pub entity_id: Entity,
}

impl Chunk {
    pub fn new(
        voxels: ChunkData,
        chunk_pos: Vec3,
        adjacent_chunks: [Option<&Chunk>; 6],
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let entity_id = commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(to_mesh(&voxels, chunk_pos, adjacent_chunks)),
                    material: materials.add(Color::rgb(0.2, 1.0, 0.3).into()),
                    transform: Transform {
                        translation: chunk_pos,
                        ..default()
                    },
                    ..default()
                },
                Wireframe,
                Name::new(format!("Chunk {chunk_pos}")),
            ))
            .id();

        Self {
            voxels,
            chunk_pos,
            entity_id,
        }
    }
    pub fn linearize(x: f32, y: f32, z: f32) -> usize {
        ((z * CHUNK_SIZE * CHUNK_SIZE) + (y * CHUNK_SIZE) + x) as usize
    }

    pub fn delinearize(pos: usize) -> Vec3 {
        Vec3::new(
            pos as f32 % CHUNK_SIZE,
            ((pos as f32 / CHUNK_SIZE) as i32 % CHUNK_SIZE as i32) as f32,
            (pos as f32 / (CHUNK_SIZE * CHUNK_SIZE)) as i32 as f32,
        )
    }
}
