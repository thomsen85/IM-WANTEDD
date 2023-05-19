use bevy::prelude::*;

use super::{components::Drone, constants::*};

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

pub fn update_drones(mut drones: Query<&mut Transform, With<Drone>>, time: Res<Time>) {
    for mut transform in drones.iter_mut() {
        transform.translation += Vec3::new(0.0, 0.0, DRONE_SPEED) * time.delta_seconds();
    }
}

pub fn tick_ping_clock(mut drones: Query<&mut Drone>, time: Res<Time>) {
    for mut drone in drones.iter_mut() {
        drone.ping_timer.tick(time.delta());
    }
}
