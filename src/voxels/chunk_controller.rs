use std::collections::HashMap;

use bevy::prelude::*;

use super::chunk::*;
use super::terrain_gen::*;
use crate::utils::*;

pub struct ChunkController {
    pub loaded_chunks: HashMap<(usize, usize, usize), Chunk>,
}

impl ChunkController {
    pub fn new() -> Self {
        let mut loaded_chunks: HashMap<(usize, usize, usize), Chunk> = HashMap::new();
        let size = 2;
        for i in 0..(size * size) {
            loaded_chunks.insert(
                (i % size, 0, i / size),
                Chunk::new(
                    gen_terrain(Vec3::new((i % size) as f32, 0., (i / size) as f32)),
                    Vec3::new((i % size) as f32, 0., (i / size) as f32),
                ),
            );
        }

        Self { loaded_chunks }
    }

    pub fn spawn_chunks(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        for (_, chunk) in self.loaded_chunks.iter() {
            commands.spawn(ChunkBundle::new(
                &chunk,
                &self.loaded_chunks,
                meshes,
                materials,
            ));
        }
    }
}
