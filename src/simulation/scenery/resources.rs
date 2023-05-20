use bevy::prelude::*;

use super::constants::GROUND_HEIGHT;

#[derive(Resource)]
pub struct GroundState {
    pub ground_height: f32,
}

impl Default for GroundState {
    fn default() -> Self {
        Self {
            ground_height: GROUND_HEIGHT,
        }
    }
}
