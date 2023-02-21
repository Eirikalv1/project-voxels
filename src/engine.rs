use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::utils::CHUNK_SIZE;
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
    commands.spawn((Camera3dBundle::default(), FlyCam, ChunkController::default()));
}

fn render_chunks(
    mut voxel_controller_query: Query<(&mut ChunkController, &Transform)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut voxel_controller = voxel_controller_query.single_mut();
    if !voxel_controller.0.chunk_loaded(IVec3::new(0, 0, 0)) {
        voxel_controller.0.load_chunk(
            IVec3::new(0, 0, 0),
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
    if !voxel_controller.0.chunk_loaded(IVec3::new(1, 0, 0)) {
        voxel_controller.0.load_chunk(
            IVec3::new(1, 0, 0),
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }

}
