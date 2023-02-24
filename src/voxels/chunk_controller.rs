use std::collections::HashMap;

use bevy::prelude::*;

use super::chunk_systems::{chunk::*, terrain_generation::generate::*};

#[derive(Resource)]
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
        self.loaded_chunks.insert(
            pos,
            Chunk::new(
                gen_terrain(pos),
                pos.as_vec3(),
                self.get_adjacent_chunk(pos),
                commands,
                meshes,
                materials,
            ),
        );
    }

    pub fn unload_chunk(&mut self, pos: IVec3, commands: &mut Commands) {
        commands.entity(self.loaded_chunks[&pos].entity_id).despawn_recursive();
        self.loaded_chunks.remove(&pos);
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
