use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

const DRONE_ROWS: usize = 10;
const DRONE_COLUMNS: usize = 10;
const DRONE_SPACING: f32 = 4.0;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system(update_drones)
        .run();
}

#[derive(Component)]
pub struct Drone {}

fn update_drones(mut drones: Query<&mut Transform, With<Drone>>) {
    for mut transform in drones.iter_mut() {
        transform.translation += Vec3::new(0.0, 0.0, 0.1);
        if transform.translation.z > 200.0 {
            transform.translation.z = -200.0;
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
        mesh: meshes.add(shape::Plane::from_size(500.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // light
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 150000.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..default()
    // });
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

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-20.0, 20.5, 50.0)
            .looking_at(Vec3::new(0.0, 0.0, 20.0), Vec3::Y),
        ..default()
    });
    for x in 0..DRONE_COLUMNS {
        for z in 0..DRONE_ROWS {
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load("untitled.glb#Scene0"),
                    transform: Transform::from_xyz(
                        x as f32 * DRONE_SPACING,
                        10.0,
                        z as f32 * DRONE_SPACING,
                    )
                    .with_scale(Vec3::splat(0.1)),
                    ..default()
                },
                Drone {},
            ));
        }
    }
}
