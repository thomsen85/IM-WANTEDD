use bevy::prelude::*;

pub mod components;
mod systems;

pub struct EmergencyBeaconsPlugin;
impl Plugin for EmergencyBeaconsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::setup);
    }
}
