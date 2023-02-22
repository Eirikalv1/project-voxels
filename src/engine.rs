use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::utils::{to_chunk_pos, RENDER_DISTANCE_RANGE};
use crate::voxels::chunk_controller::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(ChunkController::default())
        .add_startup_system_set(
            SystemSet::new()
                .with_system(spawn_pointlight)
                .with_system(init),
        )
        .add_system(render_chunks)
        .run();
}

fn spawn_pointlight(mut commands: Commands) {
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 6.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}

fn init(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), FlyCam));
}

fn render_chunks(
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

            if !chunk_controller.chunk_loaded(chunk_pos) {
                chunk_controller.load_chunk(chunk_pos, &mut commands, &mut meshes, &mut materials);
            }
        }
    }
}
