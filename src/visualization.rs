use std::{f32::consts::PI, io::Write};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

const DRONE_ROWS: usize = 10;
const DRONE_COLUMNS: usize = 10;
const DRONE_SPACING: f32 = 50.0;
const DRONE_SIZE_MUTIPLIER: f32 = 0.1;
const DRONE_HEIGHT: f32 = 10.0;
const DRONE_SPEED: f32 = 3.0;
const CONNECTION_PIPE_RADIUS: f32 = 0.04;
const GROUND_HEIGHT: f32 = -300.0;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<Fps>()
        .init_resource::<CameraState>()
        .add_startup_system(setup)
        .add_system(update_drones)
        .add_system(update_fps_counter)
        .add_system(ui_example_system)
        .add_system(update_connections)
        .add_system(update_camera)
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

#[derive(Resource)]
pub struct CameraState {
    pub relative_pos: Vec3,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            relative_pos: Vec3::new(30.0, 30.0, 30.0),
        }
    }
}

#[derive(Component)]
pub struct Connection {
    pub from: usize,
    pub to: usize,
}

fn update_camera(
    mut query: Query<(&Drone, &Transform), Without<Camera>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    camera_state: Res<CameraState>,
) {
    let mut camera_transform = camera_query.single_mut();
    let mut center = Vec3::ZERO;
    let mut count = 0;
    for (_, transform) in query.iter_mut() {
        center += transform.translation;
        count += 1;
    }
    center /= count as f32;
    camera_transform.translation = center + camera_state.relative_pos;
    camera_transform.look_at(center, Vec3::Y);
}

fn update_drones(mut drones: Query<&mut Transform, With<Drone>>, time: Res<Time>) {
    for mut transform in drones.iter_mut() {
        transform.translation += Vec3::new(0.0, 0.0, DRONE_SPEED) * time.delta_seconds();
    }
}

fn update_connections(
    drones: Query<(&Drone, &Transform), (Without<Connection>, With<Drone>)>,
    mut connections: Query<
        (&Connection, &mut Transform, Entity, &Handle<Mesh>),
        (Without<Drone>, With<Connection>),
    >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // TODO: remove unused connections
    // TODO: update length of connections
    let query_list = drones.iter().collect::<Vec<_>>();

    for (drone, transform) in drones.iter() {
        let nearby = get_nearby_drones((drone, transform), &query_list, 7.0);

        for (nearby_drone, nearby_transform) in nearby {
            if nearby_drone.id == drone.id {
                continue;
            }
            let connection = connections
                .iter_mut()
                .filter(|(connection, _, _, _)| {
                    connection.from == drone.id && connection.to == nearby_drone.id
                })
                .next();

            let diff_vec = nearby_transform.translation - transform.translation;
            let mid_vec = nearby_transform
                .translation
                .lerp(transform.translation, 0.5);
            let length = diff_vec.length() * 0.8;
            println!("drone: {}", drone.id);
            println!("nearby_drone: {}", nearby_drone.id);

            println!("length: {}", length);
            std::io::stdout().flush().unwrap();

            if let Some(mut conn) = connection {
                conn.1.translation = mid_vec;
                conn.1.rotation = Quat::from_rotation_arc(diff_vec.normalize(), Vec3::Y);
                // Optimize this
                // meshes.set(
                //     conn.3,
                //     shape::Cylinder {
                //         radius: CONNECTION_RADIUS,
                //         height: length,
                //         ..Default::default()
                //     }
                //     .into(),
                // );
            } else {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(
                            shape::Cylinder {
                                radius: CONNECTION_PIPE_RADIUS,
                                height: length,
                                ..Default::default()
                            }
                            .into(),
                        ),
                        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                        transform: Transform::from_translation(mid_vec)
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

fn ui_example_system(
    mut contexts: EguiContexts,
    fps: Res<Fps>,
    mut camera_state: ResMut<CameraState>,
) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        egui::Grid::new("My_Grid").num_columns(2).show(ui, |ui| {
            ui.label("FPS:");
            ui.label(format!("{:.2}", fps.amount));
            ui.end_row();
            ui.label("Camera pos x:");
            ui.add(egui::DragValue::new(&mut camera_state.relative_pos.x).speed(0.1));
            ui.end_row();
            ui.label("Camera pos y:");
            ui.add(egui::DragValue::new(&mut camera_state.relative_pos.y).speed(0.1));
            ui.end_row();
            ui.label("Camera pos z:");
            ui.add(egui::DragValue::new(&mut camera_state.relative_pos.z).speed(0.1));
        });
    });
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    // mut meshes: ResMut<Assets<Mesh>>,
) {
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
                        DRONE_HEIGHT,
                        z as f32 * DRONE_SPACING,
                    )
                    .with_scale(Vec3::splat(DRONE_SIZE_MUTIPLIER)),
                    ..default()
                },
                Drone {
                    id: x * DRONE_ROWS + z,
                    connections: Vec::new(),
                },
            ));
            // Messurement sphere
            // commands.spawn(PbrBundle {
            //     mesh: meshes.add(
            //         shape::UVSphere {
            //             radius: 0.5,
            //             ..Default::default()
            //         }
            //         .into(),
            //     ),
            //     material: materials.add(Color::rgba(0.3, 0.5, 0.3, 0.5).into()),
            //     transform: Transform::from_xyz(
            //         x as f32 * DRONE_SPACING,
            //         DRONE_HEIGHT,
            //         z as f32 * DRONE_SPACING,
            //     ),
            //     ..default()
            // });
        }
    }
}
