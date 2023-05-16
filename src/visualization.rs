//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_drones)
        .run();
}

#[derive(Component)]
pub struct Drone {}

fn update_drones(mut drones: Query<&mut Transform, With<Drone>>) {
    for mut transform in drones.iter_mut() {
        transform.translation += Vec3::new(0.0, 0.0, 0.1);
        if transform.translation.z > 5.0 {
            transform.translation.z = -5.0;
        }
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("untitled.glb#Scene0"),
            ..default()
        },
        Drone {},
    ));
}
