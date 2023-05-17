use bevy::prelude::*;

use crate::simulation::drones::Drone;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_system(camera_follow_drones)
            .init_resource::<CameraState>();
    }
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

fn camera_follow_drones(
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

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-20.0, 20.5, 50.0)
            .looking_at(Vec3::new(0.0, 0.0, 20.0), Vec3::Y),
        ..default()
    });
}
