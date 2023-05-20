use bevy::prelude::*;

use super::components::EmergencyBeacon;

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(10.0, -40.0, 120.0),

            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..default()
        },
        EmergencyBeacon { id: 0 },
    ));
}
