use crate::simulation::drone_connections::components::Connection;
use crate::simulation::drone_connections::constants::CONNECTION_PIPE_LENGTH_PADDING;
use crate::simulation::drone_connections::constants::CONNECTION_PIPE_RADIUS;
use crate::simulation::drones::components::Drone;
use crate::simulation::drones::constants::DRONE_CONNECTION_DISTANCE;
use bevy::prelude::*;

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
                        material: materials.add(Color::rgba(0.3, 0.5, 0.3, 0.2).into()),
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
