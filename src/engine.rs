use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::utils::{to_chunk_pos, CHUNK_SIZE};
use crate::voxels::chunk_controller::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
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
    commands.spawn((
        Camera3dBundle::default(),
        FlyCam,
        ChunkController::default(),
    ));
}

fn render_chunks(
    mut voxel_controller_query: Query<(&mut ChunkController, &Transform)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut voxel_controller = voxel_controller_query.single_mut();
    let player_pos = to_chunk_pos(voxel_controller.1.translation);

    if (player_pos.x % CHUNK_SIZE as i32 == 0
        || player_pos.y % CHUNK_SIZE as i32 == 0
        || player_pos.z % CHUNK_SIZE as i32 == 0)
        && !voxel_controller.0.chunk_loaded(player_pos)
    {
        voxel_controller
            .0
            .load_chunk(player_pos, &mut commands, &mut meshes, &mut materials);
    }
}
