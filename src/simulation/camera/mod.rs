use bevy::prelude::*;

pub mod resources;
mod systems;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::CameraState>()
            .add_startup_system(systems::setup)
            .add_system(systems::camera_follow_drones);
    }
}
