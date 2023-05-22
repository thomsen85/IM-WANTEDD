use std::{collections::VecDeque, time::Duration};

use bevy::prelude::*;

use crate::simulation::{
    drone_connections::components::Message, emergency_pings::components::EmergencyPing,
};

use super::constants::DRONE_PING_INTERVAL_MILLIS;

#[derive(Component, Debug)]
pub struct Drone {
    pub id: usize,
    pub connections: Vec<usize>,
    pub reverse: bool,
    pub ping_timer: Timer,
    pub data: Vec<EmergencyPing>,
    pub inbox: VecDeque<Message>,
    pub outbox: VecDeque<Message>,
}

impl Default for Drone {
    fn default() -> Self {
        Self {
            id: 0,
            connections: vec![],
            reverse: false,
            ping_timer: Timer::new(
                Duration::from_millis(DRONE_PING_INTERVAL_MILLIS),
                TimerMode::Once,
            ),
            data: Vec::new(),
            inbox: VecDeque::new(),
            outbox: VecDeque::new(),
        }
    }
}
