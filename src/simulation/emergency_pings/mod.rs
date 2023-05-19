use bevy::prelude::*;

use self::resources::EmergencyPingState;

pub mod components;
pub mod constants;
pub mod resources;
mod systems;

pub struct EmergencyPingsPlugin;
impl Plugin for EmergencyPingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EmergencyPingState>()
            .add_system(systems::ping_emergency_beacons)
            .add_system(systems::emergency_ping_visibility);
    }
}
