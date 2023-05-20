use bevy::prelude::*;

use super::constants::DRONE_CONNECTION_RANGE;

#[derive(Resource)]
pub struct DroneState {
    pub next_id: u32,
    pub drone_connection_range: f32,
}

impl Default for DroneState {
    fn default() -> Self {
        Self {
            next_id: 0,
            drone_connection_range: DRONE_CONNECTION_RANGE,
        }
    }
}
