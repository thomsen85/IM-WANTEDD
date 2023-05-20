use bevy::prelude::*;
use chrono::prelude::*;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct EmergencyPing {
    pub drone_id: usize,
    pub emergency_beacon_id: usize,
    pub timestamp: DateTime<chrono::offset::Local>,
    pub coordinates: Vec3,
    pub distance: f32,
}
