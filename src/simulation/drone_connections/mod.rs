use bevy::prelude::*;

pub mod components;
pub mod constants;
mod systems;
mod utils;

pub struct DroneConnectionsPlugin;
impl Plugin for DroneConnectionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(systems::update_drone_connections);
    }
}
