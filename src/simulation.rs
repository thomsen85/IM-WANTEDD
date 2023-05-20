use bevy::prelude::*;

mod camera;
mod drone_connections;
mod drones;
mod emergency_beacons;
mod emergency_pings;
mod scenery;
mod ui;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "IM-WANTEDD".to_string(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(scenery::SceneryPlugin)
        .add_plugin(ui::UIPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(drones::DronesPlugin)
        .add_plugin(drone_connections::DroneConnectionsPlugin)
        .add_plugin(emergency_beacons::EmergencyBeaconsPlugin)
        .add_plugin(emergency_pings::EmergencyPingsPlugin)
        .run();
}
