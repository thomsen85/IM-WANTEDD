use bevy::prelude::*;

pub mod resources;
mod systems;
pub struct FpsCounterPlugin;
impl Plugin for FpsCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(systems::update_fps_counter)
            .init_resource::<resources::Fps>();
    }
}
