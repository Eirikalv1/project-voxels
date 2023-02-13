use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

use super::chunk::*;
use crate::utils::*;

pub struct ChunkController {
    pub chunks: Vec<Chunk>,
}

impl ChunkController {
    pub fn new() -> Self {
        let mut chunks: Vec<Chunk> = vec![];

        chunks.push(Chunk::new(
            gen_terrain(),
            Vec3::new(0., 0., 0.)));

        Self { chunks }
    }

    pub fn spawn_chunks(
        &mut self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        for i in 0..1 {
            commands.spawn(ChunkBundle::new(
                self.chunks[i],
                &mut meshes,
                &mut materials,
            ));
        }
    }
}

fn gen_terrain() -> [VoxelType; CHUNK_SIZE_CUBED] {
    let mut chunk_data: [VoxelType; CHUNK_SIZE_CUBED] = [VoxelType::Air; CHUNK_SIZE_CUBED];

    let open_simplex = Perlin::new(1);
    for i in 0..CHUNK_SIZE_CUBED {
        let data3d = to_3d(i as f32);
        let val = open_simplex.get([22./7. * data3d.x as f64, 22./7. * data3d.y as f64, 22./7. * data3d.z as f64]);
        println!("{}", val);
        let capped_val = val.min(1.0).max(0.0);
        if capped_val == 0. {chunk_data[i] = VoxelType::Block;}
    }

    chunk_data
}