use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

const DRONE_ROWS: usize = 10;
const DRONE_COLUMNS: usize = 10;
const DRONE_SPACING: f32 = 4.0;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<Fps>()
        .add_startup_system(setup)
        .add_system(update_drones)
        .add_system(update_fps_counter)
        .add_system(ui_example_system)
        .add_system(update_connections)
        .run();
}

#[derive(Component)]
pub struct Drone {
    pub id: usize,
    pub connections: Vec<usize>,
}

#[derive(Resource, Default)]
pub struct Fps {
    pub amount: f32,
}

#[derive(Component)]
pub struct Connection {
    pub from: usize,
    pub to: usize,
}

fn update_drones(mut drones: Query<&mut Transform, With<Drone>>) {
    for mut transform in drones.iter_mut() {
        transform.translation += Vec3::new(0.0, 0.0, 0.1);
        if transform.translation.z > 200.0 {
            transform.translation.z = -200.0;
        }
    }
}

fn update_connections(
    drones: Query<(&Drone, &Transform), Without<Connection>>,
    mut connections: Query<(&Connection, &mut Transform, Entity), Without<Drone>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let query_list = drones.iter().collect::<Vec<_>>();

    for (drone, transform) in drones.iter() {
        let nearby = get_nearby_drones((drone, transform), &query_list, 5.0);
        println!("{}", nearby.len());

        for (nearby_drone, nearby_transform) in nearby {
            if nearby_drone.id == drone.id {
                continue;
            }
            let connection = connections
                .iter_mut()
                .filter(|(connection, _, _)| {
                    connection.from == drone.id && connection.to == nearby_drone.id
                })
                .next();

            let diff_vec = nearby_transform.translation - transform.translation;

            if let Some(mut conn) = connection {
                conn.1.translation = Vec3::new(
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z,
                );
                conn.1.rotation = Quat::from_rotation_arc(diff_vec.normalize(), Vec3::Y)
            } else {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(
                            shape::Cylinder {
                                radius: 0.1,
                                height: transform
                                    .translation
                                    .distance(nearby_transform.translation),
                                ..Default::default()
                            }
                            .into(),
                        ),
                        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                        transform: Transform::from_xyz(
                            transform.translation.x,
                            transform.translation.y,
                            transform.translation.z,
                        )
                        .with_rotation(Quat::from_rotation_arc(diff_vec.normalize(), Vec3::Y)),
                        ..default()
                    },
                    Connection {
                        from: drone.id,
                        to: nearby_drone.id,
                    },
                ));
            }
        }
    }
}

fn get_nearby_drones<'a>(
    current_drone: (&Drone, &Transform),
    drones: &Vec<(&'a Drone, &'a Transform)>,
    distance: f32,
) -> Vec<(&'a Drone, &'a Transform)> {
    let mut nearby_drones = Vec::new();
    for (drone, transform) in drones.iter() {
        if transform.translation.distance(current_drone.1.translation) < distance {
            nearby_drones.push((*drone, *transform));
        }
    }
    nearby_drones
}

fn update_fps_counter(time: Res<Time>, mut fps: ResMut<Fps>) {
    fps.amount = 1.0 / time.delta_seconds();
}

fn ui_example_system(mut contexts: EguiContexts, fps: Res<Fps>) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("FPS: {}", fps.amount));
    });
}
/// set up a simple 3D scene
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // // plane
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Plane::from_size(500.0).into()),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });

    commands.spawn(SceneBundle {
        scene: asset_server.load("gjerdrum.gltf#Scene0"),
        transform: Transform::from_xyz(-200.0, -1000.0, -200.0),
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
                    scene: asset_server.load("drone.glb#Scene0"),
                    transform: Transform::from_xyz(
                        x as f32 * DRONE_SPACING,
                        10.0,
                        z as f32 * DRONE_SPACING,
                    )
                    .with_scale(Vec3::splat(0.1)),
                    ..default()
                },
                Drone {
                    id: x * DRONE_ROWS + z,
                    connections: Vec::new(),
                },
            ));
        }
    }
}
