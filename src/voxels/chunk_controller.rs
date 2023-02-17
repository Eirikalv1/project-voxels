use bevy::prelude::*;

use super::chunk::*;
use super::terrain_gen::*;

pub struct ChunkController {
    pub chunks: Vec<Chunk>,
}

impl ChunkController {
    pub fn new() -> Self {
        let mut chunks: Vec<Chunk> = vec![];
        let size = 16;
        for i in 0..(size * size) {
            chunks.push(Chunk::new(
                gen_terrain(Vec3::new((i%size) as f32, 0., (i/size) as f32)),
                Vec3::new((i%size) as f32, 0., (i/size) as f32),
            ));
        }
        

        Self { chunks }
    }

    pub fn spawn_chunks(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        for i in 0..self.chunks.len() {
            commands.spawn(ChunkBundle::new(self.chunks[i].clone(), meshes, materials));
        }
    }
}
