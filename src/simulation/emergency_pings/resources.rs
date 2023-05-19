use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct EmergencyPingState {
    pub visible: bool,
}
