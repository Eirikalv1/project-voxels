use bevy::prelude::*;

use super::chunk::*;
use super::terrain_gen::*;

pub struct ChunkController {
    pub chunks: Vec<Chunk>,
}

impl ChunkController {
    pub fn new() -> Self {
        let mut chunks: Vec<Chunk> = vec![];

        chunks.push(Chunk::new(
            gen_terrain(Vec3::new(0., 0., 0.)),
            Vec3::new(0., 0., 0.),
        ));
        chunks.push(Chunk::new(
            gen_terrain(Vec3::new(1., 0., 0.)),
            Vec3::new(1., 0., 0.),
        ));
        chunks.push(Chunk::new(
            gen_terrain(Vec3::new(1., 0., 1.)),
            Vec3::new(1., 0., 1.),
        ));
        chunks.push(Chunk::new(
            gen_terrain(Vec3::new(0., 0., 1.)),
            Vec3::new(0., 0., 1.),
        ));

        Self { chunks }
    }

    pub fn spawn_chunks(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        for i in 0..self.chunks.len() {
            commands.spawn(ChunkBundle::new(self.chunks[i], meshes, materials));
        }
    }
}
