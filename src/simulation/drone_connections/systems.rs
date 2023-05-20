use crate::simulation::drone_connections::components::Connection;
use crate::simulation::drone_connections::constants::CONNECTION_PIPE_LENGTH_PADDING;
use crate::simulation::drone_connections::constants::CONNECTION_PIPE_RADIUS;
use crate::simulation::drones::components::Drone;
use crate::simulation::drones::resources::DroneState;
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
    drone_state: Res<DroneState>,
) {
    // TODO: remove unused connections
    let drones_copy = drones.iter().collect::<Vec<_>>();
    let mut frame_connections: Vec<Connection> = Vec::new();

    for (drone, transform) in drones.iter() {
        let nearby = get_nearby_drones(
            (drone, transform),
            &drones_copy,
            drone_state.drone_connection_range,
        );

        // Dangling connections
        for dangling_connections in connections
            .iter()
            .filter(|(c, _, _, _)| c.from == drone.id && !nearby.iter().any(|(d, _)| c.to == d.id))
        {
            commands.entity(dangling_connections.2).despawn();
            debug!(
                "Dangling connection from {} to {} removed",
                dangling_connections.0.from, dangling_connections.0.to
            );
        }

        // Connected Drones connections
        for mut connection in connections
            .iter_mut()
            .filter(|(c, _, _, _)| c.from == drone.id && nearby.iter().any(|(d, _)| c.to == d.id))
        {
            let (_, connected_drone_transform) = drones_copy
                .iter()
                .find(|(d, _)| d.id == connection.0.to)
                .expect("Connected drone not found");

            let (diff_vec, mid_vec, length) =
                get_connection_information(connected_drone_transform, transform);
            connection.1.translation = mid_vec;
            connection.1.scale = Vec3::new(
                CONNECTION_PIPE_RADIUS * 2.0,
                length,
                CONNECTION_PIPE_RADIUS * 2.0,
            );
            connection.1.rotation = Quat::from_rotation_arc(Vec3::Y, diff_vec.normalize());
        }

        // Unconnected_drones
        for (unconnected_drone, unconnected_drone_transform) in nearby.iter().filter(|(d, _)| {
            d.id != drone.id
                && !connections
                    .iter()
                    .any(|(c, _, _, _)| c.from == drone.id && c.to == d.id)
        }) {
            let init_message = {
                if connections
                    .iter()
                    .any(|(c, _, _, _)| c.from == unconnected_drone.id && c.to == drone.id)
                    || frame_connections
                        .iter()
                        .any(|c| c.from == unconnected_drone.id && c.to == drone.id)
                {
                    vec![]
                } else {
                    drone.data.clone()
                }
            };

            let connection = Connection {
                from: drone.id,
                to: unconnected_drone.id,
                to_be_sendt: init_message.clone(),
                new_connection: true,
                ..Default::default()
            };

            frame_connections.push(connection.clone());

            let (diff_vec, mid_vec, length) =
                get_connection_information(&unconnected_drone_transform, transform);
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
                connection,
            ));
            debug!(
                "Spawned connection between {} and {}",
                drone.id, unconnected_drone.id
            );
        }
    }
}

fn get_connection_information(
    nearby_transform: &Transform,
    transform: &Transform,
) -> (Vec3, Vec3, f32) {
    let diff_vec = nearby_transform.translation - transform.translation;
    let mid_vec = nearby_transform
        .translation
        .lerp(transform.translation, 0.5);
    let length = diff_vec.length();
    let length = f32::min(length, length - CONNECTION_PIPE_LENGTH_PADDING);
    (diff_vec, mid_vec, length)
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
            "Connection not found: Connections, {}, Msg. {:?}",
            drone_connections.iter().count(),
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
        let drone_id = drone.id;
        for message in drone.outbox.iter() {
            for mut connection in connections
                .iter_mut()
                .filter(|connection| connection.from == drone_id)
            {
                if message.from != connection.to {
                    connection
                        .to_be_sendt
                        .append(&mut message.packet_data.clone());
                }
            }
        }
        drone.outbox.clear();
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

        let message = Message {
            from: connection.from,
            packet_data: connection.to_be_sendt.clone(),
            progress: 0.0,
            init_message: connection.new_connection,
            ..Default::default()
        };

        connection.new_connection = false;
        connection.to_be_sendt.clear();

        let drone = drones
            .iter()
            .filter(|(drone, _)| drone.id == connection.from)
            .next();

        debug_assert!(drone.is_some(), "Drone not found");

        if let Some((_, drone_transform)) = drone {
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
