use bevy::prelude::*;

pub struct FpsCounterPlugin;
impl Plugin for FpsCounterPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(update_fps_counter)
        .init_resource::<Fps>()
        ;
    }
}

#[derive(Resource, Default)]
pub struct Fps {
    pub amount: f32,
}

fn update_fps_counter(time: Res<Time>, mut fps: ResMut<Fps>) {
  fps.amount = 1.0 / time.delta_seconds();
}
