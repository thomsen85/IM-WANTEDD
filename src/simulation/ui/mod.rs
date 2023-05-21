use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use self::{drone_numbering::DroneNumberingPlugin, fps_counter::FpsCounterPlugin};

use super::AppState;

mod drone_numbering;
mod fps_counter;
pub mod resources;
mod systems;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::UiState>()
            .add_plugin(EguiPlugin)
            .add_plugin(FpsCounterPlugin)
            .add_plugin(DroneNumberingPlugin)
            // .add_system(systems::controll_ui)
            // .add_system(systems::console_ui)
            // .add_system(systems::drone_detail_ui);
            .add_systems(
                (
                    systems::controll_ui,
                    systems::console_ui,
                    systems::drone_detail_ui,
                )
                    .in_set(OnUpdate(AppState::InSimulation)),
            );
    }
}
