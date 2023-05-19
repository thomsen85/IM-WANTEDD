use std::time::Duration;

use bevy::prelude::*;

use super::constants::DRONE_PING_INTERVAL_MILLIS;

#[derive(Component, Debug)]
pub struct Drone {
    pub id: usize,
    pub connections: Vec<usize>,
    pub ping_timer: Timer,
}

impl Default for Drone {
    fn default() -> Self {
        Self {
            id: 0,
            connections: vec![],
            ping_timer: Timer::new(
                Duration::from_millis(DRONE_PING_INTERVAL_MILLIS),
                TimerMode::Repeating,
            ),
        }
    }
}
