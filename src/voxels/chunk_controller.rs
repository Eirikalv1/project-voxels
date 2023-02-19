use std::collections::HashMap;

use bevy::prelude::*;

use crate::utils::tuple_to_vec3;

use super::chunk::*;
use super::terrain_gen::*;

pub struct ChunkController {
    pub loaded_chunks: HashMap<(i32, i32, i32), Chunk>,
}

impl ChunkController {
    pub fn new() -> Self {
        let loaded_chunks: HashMap<(i32, i32, i32), Chunk> = HashMap::new();

        Self { loaded_chunks }
    }

    pub fn load_chunk(
        &mut self,
        pos: (i32, i32, i32),
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        if self.loaded_chunks.contains_key(&pos) {
            return;
        }

        let chunk = Chunk::new(gen_terrain(tuple_to_vec3(pos)), tuple_to_vec3(pos));
        self.loaded_chunks.insert(pos, chunk);

        commands.spawn(ChunkBundle::new(
            &self.loaded_chunks[&pos],
            self.get_adjacent_chunk(tuple_to_vec3(pos)),
            meshes,
            materials,
        ));
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

impl Default for ChunkController {
    fn default() -> Self {
        Self::new()
    }
}
