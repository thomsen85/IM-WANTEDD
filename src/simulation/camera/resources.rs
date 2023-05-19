use bevy::prelude::*;

#[derive(Resource)]
pub struct CameraState {
    pub relative_pos: Vec3,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            relative_pos: Vec3::new(50.0, 30.0, 40.0),
        }
    }
}
