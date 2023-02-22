use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::voxels::{chunk_controller::*, render_system::render_chunks};

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

