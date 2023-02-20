use std::collections::HashMap;

use bevy::prelude::*;

use crate::utils::to_fvec3;

use super::chunk::*;
use super::terrain_gen::*;

#[derive(Component)]
pub struct ChunkController {
    pub loaded_chunks: HashMap<IVec3, Chunk>,
}

impl ChunkController {
    pub fn new() -> Self {
        let loaded_chunks: HashMap<IVec3, Chunk> = HashMap::new();

        Self { loaded_chunks }
    }

    pub fn chunk_loaded(&self, pos: IVec3) -> bool {
        self.loaded_chunks.contains_key(&pos)
    }

    pub fn load_chunk(
        &mut self,
        pos: IVec3,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let chunk = Chunk::new(gen_terrain(to_fvec3(pos)), to_fvec3(pos));
        self.loaded_chunks.insert(pos, chunk);

        commands.spawn(ChunkBundle::new(
            &self.loaded_chunks[&pos],
            self.get_adjacent_chunk(pos),
            meshes,
            materials,
        ));
    }

    fn get_adjacent_chunk(&self, pos: IVec3) -> [Option<&Chunk>; 6] {
        [
            self.loaded_chunks.get(&IVec3::new(pos.x + 1, pos.y, pos.z)),
            self.loaded_chunks.get(&IVec3::new(pos.x - 1, pos.y, pos.z)),
            self.loaded_chunks.get(&IVec3::new(pos.x, pos.y + 1, pos.z)),
            self.loaded_chunks.get(&IVec3::new(pos.x, pos.y - 1, pos.z)),
            self.loaded_chunks.get(&IVec3::new(pos.x, pos.y, pos.z + 1)),
            self.loaded_chunks.get(&IVec3::new(pos.x, pos.y, pos.z - 1)),
        ]
    }
}

impl Default for ChunkController {
    fn default() -> Self {
        Self::new()
    }
}
