use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::voxels::chunk_controller::*;

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
                .with_system(init),
        )
        .add_system(move_player)
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
    commands.spawn((SpatialBundle::default(), Player, ChunkController::default()));
}

#[derive(Component)]
struct Player;

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = player_query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        direction.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::LShift) {
        direction.y -= 1.0;
    }
    player_transform.translation += direction;
}

fn render_chunks(
    mut voxel_controller_query: Query<(&mut ChunkController, &Transform), With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut voxel_controller = voxel_controller_query.single_mut();
    if !voxel_controller.0.chunk_loaded((0, 0, 0)) {
        voxel_controller
            .0
            .load_chunk((0, 0, 0), &mut commands, &mut meshes, &mut materials);
    }
}
