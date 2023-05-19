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
        .add_plugins(DefaultPlugins)
        .add_plugin(scenery::SceneryPlugin)
        .add_plugin(ui::UIPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(drones::DronesPlugin)
        .add_plugin(drone_connections::DroneConnectionsPlugin)
        .add_plugin(emergency_beacons::EmergencyBeaconsPlugin)
        .add_plugin(emergency_pings::EmergencyPingsPlugin)
        .run();
}
