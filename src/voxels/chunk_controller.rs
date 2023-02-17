use std::collections::HashMap;

use bevy::prelude::*;

use super::chunk::*;
use super::terrain_gen::*;

pub struct ChunkController {
    pub chunks: HashMap<(usize, usize, usize), Chunk>,
}

impl ChunkController {
    pub fn new() -> Self {
        let mut chunks: HashMap<(usize, usize, usize), Chunk> = HashMap::new();
        let size = 2;
        for i in 0..(size * size) {
            chunks.insert(
                (i % size, 0, i / size),
                Chunk::new(
                    gen_terrain(Vec3::new((i % size) as f32, 0., (i / size) as f32)),
                    Vec3::new((i % size) as f32, 0., (i / size) as f32),
                ),
            );
        }

        Self { chunks }
    }

    pub fn spawn_chunks(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        for (_, chunk) in self.chunks.iter() {
            commands.spawn(ChunkBundle::new(&chunk, meshes, materials));
        }
    }
}
