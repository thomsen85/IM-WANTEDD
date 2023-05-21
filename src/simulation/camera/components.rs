use bevy::prelude::*;

#[derive(Component)]
pub struct OrbitCamera {
    pub radius: f32,
    pub upside_down: bool,
    pub rotation_offset: Quat,
    pub target: Option<Entity>,
}
