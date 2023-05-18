use bevy::prelude::*;

#[derive(Component)]
pub struct Drone {
    pub id: usize,
    pub connections: Vec<usize>,
}
