use bevy::prelude::*;

pub mod components;
pub mod constants;
pub mod resources;
mod systems;

pub struct DronesPlugin;
impl Plugin for DronesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::setup)
            .init_resource::<resources::DroneState>()
            .add_system(systems::update_drones)
            .add_system(systems::tick_ping_clock)
            .add_system(systems::handle_inbox);
    }
}
