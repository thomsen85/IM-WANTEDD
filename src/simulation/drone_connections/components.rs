use bevy::prelude::*;

#[derive(Component)]
pub struct Connection {
    pub from: usize,
    pub to: usize,
}
