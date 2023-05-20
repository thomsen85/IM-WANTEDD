use bevy::prelude::*;

use crate::simulation::ui::fps_counter::resources::Fps;

pub fn update_fps_counter(time: Res<Time>, mut fps: ResMut<Fps>) {
    fps.amount = 1.0 / time.delta_seconds();
}
