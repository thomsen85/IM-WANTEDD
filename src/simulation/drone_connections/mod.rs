use bevy::prelude::*;

pub mod components;
pub mod constants;
mod systems;
mod utils;

pub struct DroneConnectionsPlugin;
impl Plugin for DroneConnectionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(systems::update_drone_connections)
            .add_system(systems::create_message_spheres)
            .add_system(systems::tick_messages)
            .add_system(systems::move_message_spheres)
            .add_system(systems::add_messages_to_connection_queue)
            .add_system(systems::remove_message_spheres_and_add_message);
    }
}
