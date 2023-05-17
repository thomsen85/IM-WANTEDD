mod camera;
mod drone_connections;
mod drones;
mod ui;

use std::f32::consts::PI;

use bevy::prelude::*;

const GROUND_HEIGHT: f32 = -300.0;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ui::UIPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(drones::DronesPlugin)
        .add_plugin(drone_connections::DroneConnectionsPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("gjerdrum.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, GROUND_HEIGHT, 0.0),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}
