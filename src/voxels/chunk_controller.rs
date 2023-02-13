use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;

use super::chunk::*;
use crate::utils::*;

pub struct ChunkController {
    pub chunks: Vec<Chunk>,
}

impl ChunkController {
    pub fn new() -> Self {
        let mut chunks: Vec<Chunk> = vec![];

        chunks.push(Chunk::new(
            [VoxelType::Block; CHUNK_SIZE_CUBED],
            Vec3::new(0., 0., 0.),
        ));

        Self { chunks }
    }

    pub fn spawn_chunks(
        &mut self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(self.chunks[0].into()),
                material: materials.add(Color::rgb(0.2, 0.2, 0.7).into()),
                transform: Transform {
                    translation: self.chunks[0].world_pos,
                    ..default()
                },
                ..default()
            },
            Wireframe,
        ));
    }
}
