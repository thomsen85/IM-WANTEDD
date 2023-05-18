use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub struct DroneNumberingPlugin;
impl Plugin for DroneNumberingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::DroneNumberingState>()
            .add_system(systems::update_drone_numbering)
            .add_system(systems::drone_numbering_toggle);
    }
}
