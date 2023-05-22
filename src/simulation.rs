use bevy::prelude::*;

use self::main_menu::components::ScenarioChosen;

mod camera;
mod drone_connections;
mod drones;
mod emergency_beacons;
mod emergency_pings;
mod main_menu;
mod scenery;
mod ui;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InSimulation,
}

#[derive(Resource)]
pub struct SimulationScenario {
    pub scenario: ScenarioChosen,
}

impl Default for SimulationScenario {
    fn default() -> Self {
        Self {
            scenario: ScenarioChosen::SingleBeacon,
        }
    }
}

pub fn main() {
    App::new()
        .add_state::<AppState>()
        .init_resource::<SimulationScenario>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "IM-WANTEDD".to_string(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(scenery::SceneryPlugin)
        .add_plugin(ui::UIPlugin)
        .add_plugin(drones::DronesPlugin)
        .add_plugin(drone_connections::DroneConnectionsPlugin)
        .add_plugin(emergency_beacons::EmergencyBeaconsPlugin)
        .add_plugin(emergency_pings::EmergencyPingsPlugin)
        .add_plugin(camera::CameraPlugin)
        .run();
}
