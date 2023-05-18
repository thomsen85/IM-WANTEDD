use bevy::prelude::*;

pub struct EmergencyBeaconsPlugin;
impl Plugin for EmergencyBeaconsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_emergency_beacons);
    }
}

#[derive(Component)]
pub struct EmergencyBeacon {
    pub id: usize,
}

fn setup_emergency_beacons(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..default()
        },
        EmergencyBeacon { id: 0 },
    ));
}
