use std::collections::VecDeque;

use bevy::prelude::*;

use crate::simulation::emergency_pings::components::EmergencyPing;

#[derive(Component)]
pub struct Connection {
    pub from: usize,
    pub to: usize,
    pub to_be_sendt: VecDeque<Message>,
    pub message: Option<Entity>,
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            from: 0,
            to: 0,
            to_be_sendt: VecDeque::new(),
            message: None,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Message {
    pub packet_data: EmergencyPing,
    pub progress: f32,
    pub from: usize,
}
