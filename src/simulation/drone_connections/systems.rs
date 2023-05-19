use crate::simulation::drone_connections::components::Connection;
use crate::simulation::drone_connections::constants::CONNECTION_PIPE_LENGTH_PADDING;
use crate::simulation::drone_connections::constants::CONNECTION_PIPE_RADIUS;
use crate::simulation::drones::components::Drone;
use crate::simulation::drones::constants::DRONE_CONNECTION_DISTANCE;
use bevy::prelude::*;

use super::components::Message;
use super::constants::MESSAGE_SEND_TIME;
use super::utils::get_nearby_drones;

pub fn update_drone_connections(
    drones: Query<(&Drone, &Transform), (Without<Connection>, With<Drone>)>,
    mut connections: Query<
        (&Connection, &mut Transform, Entity, &Handle<Mesh>),
        (Without<Drone>, With<Connection>),
    >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
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
            let length = diff_vec.length();
            let length = f32::min(length, length - CONNECTION_PIPE_LENGTH_PADDING);

            if let Some(mut conn) = connection {
                conn.1.translation = mid_vec;
                conn.1.scale = Vec3::new(
                    CONNECTION_PIPE_RADIUS * 2.0,
                    length,
                    CONNECTION_PIPE_RADIUS * 2.0,
                );
                conn.1.rotation = Quat::from_rotation_arc(Vec3::Y, diff_vec.normalize());
            } else {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(shape::Cylinder::default().into()),
                        material: materials.add(Color::rgba(0.3, 0.5, 0.3, 0.2).into()),
                        transform: Transform::from_translation(mid_vec)
                            .with_scale(Vec3::new(
                                CONNECTION_PIPE_RADIUS * 2.0,
                                length,
                                CONNECTION_PIPE_RADIUS * 2.0,
                            ))
                            .with_rotation(Quat::from_rotation_arc(diff_vec.normalize(), Vec3::Y)),
                        ..default()
                    },
                    Connection {
                        from: drone.id,
                        to: nearby_drone.id,
                        ..Default::default()
                    },
                ));
            }
        }
    }
}

pub fn tick_messages(mut messages: Query<&mut Message>, time: Res<Time>) {
    let time_percentage = time.delta_seconds() / MESSAGE_SEND_TIME.as_secs_f32();
    for mut message in messages.iter_mut() {
        message.progress += time_percentage;
    }
}

pub fn move_message_spheres(
    mut messages: Query<(Entity, &Message, &mut Transform), (With<Message>, Without<Connection>)>,
    drone_connections: Query<(&Connection, &Transform), Without<Message>>,
    drones: Query<(&Drone, &Transform), (With<Drone>, Without<Message>)>,
) {
    for (message_id, message, mut transform) in messages.iter_mut() {
        let connection = drone_connections
            .iter()
            .filter(|(connection, _)| {
                if let Some(m) = connection.message {
                    m == message_id
                } else {
                    false
                }
            })
            .next();

        debug_assert!(
            connection.is_some(),
            "Connection not found: Msg. {:?}",
            message
        );

        if let Some((connection, connection_transform)) = connection {
            let drone = drones
                .iter()
                .filter(|(drone, _)| drone.id == connection.from)
                .next();

            debug_assert!(drone.is_some(), "Drone not found");

            if let Some((_, drone_transform)) = drone {
                let position = drone_transform
                    .translation
                    .lerp(connection_transform.translation, message.progress * 2.0);

                transform.translation = position;
                dbg!(position);
            }
        }
    }
}

pub fn add_messages_to_connection_queue(
    mut connections: Query<&mut Connection>,
    mut drones: Query<&mut Drone>,
) {
    for mut drone in drones.iter_mut() {
        if drone.outbox.is_empty() {
            continue;
        }
        let message = drone.outbox.pop_front().unwrap();

        for mut connection in connections
            .iter_mut()
            .filter(|connection| connection.from == drone.id)
        {
            if message.from != connection.to {
                connection.to_be_sendt.push_back(Message {
                    packet_data: message.packet_data.clone(),
                    progress: 0.0,
                    from: drone.id,
                });
            }
        }
    }
}

pub fn create_message_spheres(
    mut commands: Commands,
    mut connections: Query<&mut Connection>,
    drones: Query<(&Drone, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for connection_id in 0..connections.iter().count() {
        let read_connection = connections.iter().nth(connection_id).unwrap();
        let reverse_connection = connections
            .iter()
            .filter(|connection| {
                connection.from == read_connection.to && connection.to == read_connection.from
            })
            .next()
            .expect("Reverse connection not found");

        if reverse_connection.message.is_some() {
            continue;
        }

        let mut connection = connections.iter_mut().nth(connection_id).unwrap();
        if connection.to_be_sendt.is_empty() {
            continue;
        }
        if connection.message.is_some() {
            continue;
        }

        let message = connection.to_be_sendt.pop_front().unwrap();

        let drone = drones
            .iter()
            .filter(|(drone, _)| drone.id == connection.from)
            .next();

        println!("Creating message sphere");

        if let Some((drone, drone_transform)) = drone {
            let message = commands.spawn((
                PbrBundle {
                    mesh: meshes.add(
                        shape::UVSphere {
                            radius: 3.0,
                            ..Default::default()
                        }
                        .into(),
                    ),
                    material: materials.add(Color::rgba(0.3, 0.5, 0.3, 1.0).into()),
                    transform: drone_transform.clone(),
                    ..default()
                },
                message,
            ));
            connection.message = Some(message.id());
        }
    }
}

pub fn remove_message_spheres_and_add_message(
    mut commands: Commands,
    messages: Query<(Entity, &Message)>,
    mut connections: Query<&mut Connection>,
    mut drones: Query<&mut Drone>,
) {
    for (message_id, message) in messages.iter() {
        if message.progress < 1.0 {
            continue;
        }
        let connection = connections
            .iter_mut()
            .filter(|connection| {
                if let Some(m) = connection.message {
                    m == message_id
                } else {
                    false
                }
            })
            .next();

        debug_assert!(connection.is_some(), "Connection not found");

        if let Some(mut connection) = connection {
            let drone = drones
                .iter_mut()
                .filter(|drone| drone.id == connection.to)
                .next();

            if let Some(mut drone) = drone {
                drone.inbox.push_back(message.clone());
            }
            connection.message = None;
        }

        commands.entity(message_id).despawn();
    }
}
