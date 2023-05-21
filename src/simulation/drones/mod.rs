use crate::simulation::AppState::InSimulation;
use bevy::prelude::*;

pub mod components;
pub mod constants;
pub mod resources;
mod systems;

pub struct DronesPlugin;
impl Plugin for DronesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::DroneState>()
            .add_system(systems::setup.in_schedule(OnEnter(InSimulation)))
            // .add_system(systems::update_drones)
            // .add_system(systems::tick_ping_clock)
            // .add_system(systems::handle_inbox);
            .add_systems(
                (
                    systems::update_drones,
                    systems::tick_ping_clock,
                    systems::handle_inbox,
                )
                    .in_set(OnUpdate(InSimulation)),
            );
    }
}
