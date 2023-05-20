use bevy::prelude::*;

use crate::simulation::emergency_pings::components::EmergencyPing;

#[derive(Component)]
pub struct Connection {
    pub from: usize,
    pub to: usize,
    pub to_be_sendt: Vec<EmergencyPing>,
    pub message: Option<Entity>,
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            from: 0,
            to: 0,
            to_be_sendt: Vec::new(),
            message: None,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Message {
    pub packet_data: Vec<EmergencyPing>,
    pub progress: f32,
    pub from: usize,
    pub init_message: bool,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            packet_data: Vec::new(),
            progress: 0.0,
            from: 0,
            init_message: false,
        }
    }
}
