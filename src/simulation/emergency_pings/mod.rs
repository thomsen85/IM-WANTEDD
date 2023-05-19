use bevy::prelude::*;

pub mod components;
pub mod constants;
pub mod resources;
mod systems;

pub struct EmergencyPingsPlugin;
impl Plugin for EmergencyPingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(systems::ping_emergency_beacons);
    }
}
