use bevy::prelude::*;

pub struct MainMenuPlugin;

mod components;
mod constants;
mod systems;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::setup);
    }
}
