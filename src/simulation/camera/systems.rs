use bevy::prelude::*;

use crate::simulation::camera::resources::CameraState;
use crate::simulation::drones::Drone;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-20.0, 20.5, 50.0)
            .looking_at(Vec3::new(0.0, 0.0, 20.0), Vec3::Y),
        ..default()
    });
}

/// Makes the camera follow the drones.
pub fn camera_follow_drones(
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
