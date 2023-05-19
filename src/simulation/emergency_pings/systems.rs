use std::time::Instant;

use crate::simulation::{
    drones::components::Drone, emergency_beacons::components::EmergencyBeacon,
};
use bevy::prelude::*;

use super::{components::EmergencyPing, constants::EMERGENCY_PING_DISTANCE};

pub fn ping_emergency_beacons(
    mut commands: Commands,
    drones: Query<(&Drone, &Transform)>,
    emergency_beacons: Query<(&EmergencyBeacon, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (drone, drone_transform) in drones.iter() {
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

            println!("Ping! {:#?} {:#?}", drone, emergency_beacon);

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::UVSphere {
                        radius: distance,
                        ..Default::default()
                    })),
                    material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.1).into()),
                    transform: Transform::from_translation(drone_transform.translation),
                    ..Default::default()
                },
                EmergencyPing {
                    drone_id: drone.id,
                    emergency_beacon_id: emergency_beacon.id,
                    timestamp: Instant::now(),
                    coordinates: drone_transform.translation,
                    distance,
                },
            ));
        }
    }
}
