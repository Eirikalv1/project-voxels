use bevy::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{
    utils::to_world_pos,
    voxels::{chunk_controller::*, render_system::render_chunks},
};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .insert_resource(ChunkController::default())
        .add_startup_system_set(SystemSet::new().with_system(init))
        .add_system(render_chunks)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform {
                translation: to_world_pos(Vec3::ZERO, IVec3::new(2, 3, 2)),
                ..default()
            },
            ..default()
        },
        FlyCam,
    ));
}
