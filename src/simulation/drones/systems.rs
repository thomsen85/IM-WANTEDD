use crate::simulation::drone_connections::components::Message;

use super::{components::Drone, constants::*};
use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    ..default()
                },
            ));
        }
    }
}

pub fn update_drones(mut drones: Query<(&Drone, &mut Transform)>, time: Res<Time>) {
    for (drone, mut transform) in drones.iter_mut() {
        let random_num = ((drone.id * 127) % 19 + 5) as f32 / 10.0;
        transform.translation += Vec3::new(
            0.1 * f32::sin(time.elapsed_seconds() * random_num) * time.delta_seconds(),
            1.0 * f32::sin(time.elapsed_seconds() * random_num) * time.delta_seconds(),
            (0.1 * f32::sin(time.elapsed_seconds() * random_num) + DRONE_SPEED)
                * time.delta_seconds(),
        )
    }
}

pub fn tick_ping_clock(mut drones: Query<&mut Drone>, time: Res<Time>) {
    for mut drone in drones.iter_mut() {
        drone.ping_timer.tick(time.delta());
    }
}

pub fn handle_inbox(mut drones: Query<&mut Drone>) {
    for mut drone in drones.iter_mut() {
        if drone.inbox.is_empty() {
            continue;
        }
        let message = drone.inbox.pop_front().unwrap();
        trace!("Drone {} received message {:?}", drone.id, message);

        let mut filtered_message = Message {
            from: message.from,
            ..Default::default()
        };
        for emergency_ping in message.packet_data {
            if !drone.data.contains(&emergency_ping) {
                drone.data.push(emergency_ping.clone());
                filtered_message.packet_data.push(emergency_ping);
            }
        }
        drone.outbox.push_back(filtered_message);
    }
}
