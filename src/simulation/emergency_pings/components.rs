use std::time::Instant;

use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct EmergencyPing {
    pub drone_id: usize,
    pub emergency_beacon_id: usize,
    pub timestamp: Instant,
    pub coordinates: Vec3,
    pub distance: f32,
}
