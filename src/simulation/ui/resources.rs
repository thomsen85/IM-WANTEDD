use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct UiState {
    pub show_drone_detail: bool,
    pub current_drone: Option<Entity>,
}
