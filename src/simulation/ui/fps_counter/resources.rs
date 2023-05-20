use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Fps {
    pub amount: f32,
}
