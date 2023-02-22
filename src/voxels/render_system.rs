use bevy::prelude::*;
use bevy_flycam::FlyCam;

use crate::utils::{to_chunk_pos, RENDER_DISTANCE_RANGE, RENDER_DISTANCE_SQUARED};

use super::chunk_controller::ChunkController;

pub fn render_chunks(
    mut player_query: Query<&Transform, With<FlyCam>>,
    mut chunk_controller: ResMut<ChunkController>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = player_query.single_mut();
    let player_pos = to_chunk_pos(player.translation);

    for x in RENDER_DISTANCE_RANGE {
        for z in RENDER_DISTANCE_RANGE {
            let chunk_pos = IVec3::new(player_pos.x + x, player_pos.y, player_pos.z + z);
            let distance_sq = i32::pow(Vec3::length((chunk_pos - player_pos).as_vec3()) as i32, 2);

            if distance_sq <= RENDER_DISTANCE_SQUARED && !chunk_controller.chunk_loaded(chunk_pos) {
                chunk_controller.load_chunk(chunk_pos, &mut commands, &mut meshes, &mut materials);
            }
        }
    }

    let mut chunks_to_unload: Vec<IVec3> = vec![];
    for loaded_chunk in chunk_controller.loaded_chunks.iter() {
        let distance_sq = i32::pow(
            Vec3::length((*loaded_chunk.0 - player_pos).as_vec3()) as i32,
            2,
        );

        if distance_sq > RENDER_DISTANCE_SQUARED {
            chunks_to_unload.push(*loaded_chunk.0);
        }
    }

    for chunk_to_unload in chunks_to_unload {
        chunk_controller.unload_chunk(chunk_to_unload, &mut commands);
    }
}
