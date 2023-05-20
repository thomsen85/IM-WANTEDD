use bevy::prelude::*;

pub mod components;
pub mod constants;
pub mod resources;
mod systems;

pub struct SceneryPlugin;
impl Plugin for SceneryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::GroundState>()
            .add_startup_system(systems::setup)
            .add_system(systems::update_ground_height);
    }
}
