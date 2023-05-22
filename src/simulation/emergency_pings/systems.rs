use crate::simulation::{
    drone_connections::components::Message, drones::components::Drone,
    emergency_beacons::components::EmergencyBeacon,
};
use bevy::prelude::*;

use super::{
    components::EmergencyPing, constants::EMERGENCY_PING_DISTANCE, resources::EmergencyPingState,
};

pub fn ping_emergency_beacons(
    mut commands: Commands,
    mut drones: Query<(&mut Drone, &Transform)>,
    emergency_beacons: Query<(&EmergencyBeacon, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut drone, drone_transform) in drones.iter_mut() {
        let mut has_pinged = false;
        for (emergency_beacon, emergency_beacon_transform) in emergency_beacons.iter() {
            let distance = drone_transform
                .translation
                .distance(emergency_beacon_transform.translation);
            if distance > EMERGENCY_PING_DISTANCE {
                continue;
            }

            if !drone.ping_timer.finished() {
                continue;
            }

            trace!("Ping! Drone: {} -> {}", drone.id, emergency_beacon.id);
            let emergency_ping = EmergencyPing {
                drone_id: drone.id,
                emergency_beacon_id: emergency_beacon.id,
                timestamp: chrono::Local::now(),
                coordinates: drone_transform.translation,
                distance,
            };

            let drone_id = drone.id;

            drone.outbox.push_back(Message {
                packet_data: vec![emergency_ping.clone()],
                progress: 0.0,
                from: drone_id,
                ..Default::default()
            });

            drone.data.push(emergency_ping.clone());

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::UVSphere {
                        radius: distance,
                        ..Default::default()
                    })),
                    material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.1).into()),
                    transform: Transform::from_translation(drone_transform.translation),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
                emergency_ping,
            ));
            has_pinged = true;
        }
        if has_pinged {
            drone.ping_timer.reset();
        }
    }
}

pub fn emergency_ping_visibility(
    emergency_ping_state: Res<EmergencyPingState>,
    mut emergency_pings: Query<&mut Visibility, With<EmergencyPing>>,
) {
    for mut emergency_ping in emergency_pings.iter_mut() {
        if emergency_ping_state.visible {
            *emergency_ping = Visibility::Visible;
        } else {
            *emergency_ping = Visibility::Hidden;
        }
    }
}
