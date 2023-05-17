use std::io::Write;

use bevy::prelude::*;

use crate::simulation::drones::{Drone, DRONE_CONNECTION_DISTANCE};

pub struct DroneConnectionsPlugin;
impl Plugin for DroneConnectionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_drone_connections);
    }
}

const CONNECTION_PIPE_RADIUS: f32 = 0.04;

#[derive(Component)]
pub struct Connection {
    pub from: usize,
    pub to: usize,
}

fn update_drone_connections(
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
        let nearby = get_nearby_drones((drone, transform), &query_list, DRONE_CONNECTION_DISTANCE);

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
