use bevy::prelude::*;

const DRONE_ROWS: usize = 3;
const DRONE_COLUMNS: usize = 3;
const DRONE_SPACING: f32 = 30.0;
const DRONE_SIZE_MUTIPLIER: f32 = 0.1;
const DRONE_HEIGHT: f32 = 10.0;
const DRONE_SPEED: f32 = 3.0;
pub const DRONE_CONNECTION_DISTANCE: f32 = 45.0;

pub struct DronesPlugin;
impl Plugin for DronesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_drones)
            .add_system(update_drones);
    }
}

#[derive(Component)]
pub struct Drone {
    pub id: usize,
    pub connections: Vec<usize>,
}

fn update_drones(mut drones: Query<&mut Transform, With<Drone>>, time: Res<Time>) {
    for mut transform in drones.iter_mut() {
        transform.translation += Vec3::new(0.0, 0.0, DRONE_SPEED) * time.delta_seconds();
    }
}

fn setup_drones(mut commands: Commands, asset_server: Res<AssetServer>) {
    for x in 0..DRONE_COLUMNS {
        for z in 0..DRONE_ROWS {
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load("drone.glb#Scene0"),
                    transform: Transform::from_xyz(
                        x as f32 * DRONE_SPACING,
                        DRONE_HEIGHT,
                        z as f32 * DRONE_SPACING,
                    )
                    .with_scale(Vec3::splat(DRONE_SIZE_MUTIPLIER)),
                    ..default()
                },
                Drone {
                    id: x * DRONE_ROWS + z,
                    connections: Vec::new(),
                },
            ));
        }
    }
}
