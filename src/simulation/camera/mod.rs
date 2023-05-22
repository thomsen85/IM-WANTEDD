use bevy::prelude::*;

use super::AppState;

pub mod components;
mod systems;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::setup)
            .add_system(systems::camera_follow_entity)
            .add_system(systems::orbit_camera.run_if(in_state(AppState::InSimulation)));
    }
}
