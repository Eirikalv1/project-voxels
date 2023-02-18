use std::collections::HashMap;

use bevy::prelude::*;

use super::chunk::*;
use super::terrain_gen::*;

pub struct ChunkController {
    pub loaded_chunks: HashMap<(i32, i32, i32), Chunk>,
}

impl ChunkController {
    pub fn new() -> Self {
        let mut loaded_chunks: HashMap<(i32, i32, i32), Chunk> = HashMap::new();
        let size = 4;
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
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        for (pos3d, chunk) in self.loaded_chunks.iter() {
            commands.spawn(ChunkBundle::new(
                &chunk,
                self.get_adjacent_chunk(Vec3::new(pos3d.0 as f32, pos3d.1 as f32, pos3d.2 as f32)),
                meshes,
                materials,
            ));
        }
    }

    fn get_adjacent_chunk(&self, pos3d: Vec3) -> [Option<&Chunk>; 6] {
        let pos: (i32, i32, i32) = (pos3d.x as i32, pos3d.y as i32, pos3d.z as i32);

        [
            self.loaded_chunks.get(&(pos.0 + 1, pos.1, pos.2)),
            self.loaded_chunks.get(&(pos.0 - 1, pos.1, pos.2)),
            self.loaded_chunks.get(&(pos.0, pos.1 + 1, pos.2)),
            self.loaded_chunks.get(&(pos.0, pos.1 - 1, pos.2)),
            self.loaded_chunks.get(&(pos.0, pos.1, pos.2 + 1)),
            self.loaded_chunks.get(&(pos.0, pos.1, pos.2 - 1)),
        ]
    }
}
