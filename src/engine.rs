use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::voxel;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(PlayerPlugin)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_startup_system_set(
            SystemSet::new()
                .with_system(spawn_pointlight)
                .with_system(spawn_box),
        )
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
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}

fn spawn_box(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(voxel::VoxelBox::new(3).into()),
                material: materials.add(Color::rgb(0.2, 0.4, 0.7).into()),
                ..Default::default()
            },
            Wireframe,
        ))
        .insert(Name::new("Box"));
}
