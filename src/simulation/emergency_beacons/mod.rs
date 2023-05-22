use bevy::prelude::*;

use super::AppState;

pub mod components;
mod systems;

pub struct EmergencyBeaconsPlugin;
impl Plugin for EmergencyBeaconsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(systems::setup.in_schedule(OnEnter(AppState::InSimulation)));
    }
}
