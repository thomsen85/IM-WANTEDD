use crate::simulation::drones::components::Drone;
use crate::simulation::ui::drone_numbering::components::DroneNumbering;
use crate::simulation::ui::drone_numbering::resources::DroneNumberingState;
use bevy::prelude::*;

pub fn update_drone_numbering(
    drones: Query<(Entity, &Transform, &Drone), (With<Drone>, Without<DroneNumbering>)>,
    mut drone_numbering: Query<(Entity, &DroneNumbering, &mut Style)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    let font = asset_server.load("fonts/roboto.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let camera = camera.get_single().unwrap();
    let camera_transform = camera.1;
    let camera = camera.0;

    // Is dangeling Text
    for (entity, drone_numbering, _) in drone_numbering.iter_mut() {
        if drones.get(drone_numbering.drone).is_err() {
            commands.entity(entity).despawn();
        }
    }

    // Is connected Text
    for (_, drone_numbering, mut text_styling) in drone_numbering.iter_mut() {
        let drone = drones.get(drone_numbering.drone).unwrap();
        let drone_transform = drone.1;
        let drone_translation = drone_transform.translation;
        let drone_translation = Vec3::new(
            drone_translation.x,
            drone_translation.y + 1.0,
            drone_translation.z,
        );

        if let Some(screen_coordinates) =
            camera.world_to_viewport(camera_transform, drone_translation)
        {
            text_styling.position = UiRect {
                left: Val::Px(screen_coordinates.x),
                bottom: Val::Px(screen_coordinates.y),
                ..Default::default()
            };
        }
    }

    // Drone Does not have text
    // First quick optimization check
    if drone_numbering.iter().count() == drones.iter().count() {
        return;
    }

    for (entity, drone_transform, drone) in drones.iter().filter(|(entity, _, _)| {
        !drone_numbering
            .iter()
            .any(|(_, drone_text, _)| drone_text.drone == *entity)
    }) {
        let drone_translation = drone_transform.translation;
        let drone_translation = Vec3::new(
            drone_translation.x,
            drone_translation.y + 1.0,
            drone_translation.z,
        );
        if let Some(screen_coordinates) =
            camera.world_to_viewport(camera_transform, drone_translation)
        {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(drone.id.to_string(), text_style.clone()),
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(screen_coordinates.x),
                            top: Val::Px(screen_coordinates.y),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                DroneNumbering { drone: entity },
            ));
        }
    }
}

pub fn drone_numbering_toggle(
    drone_numbering_state: Res<DroneNumberingState>,
    mut drone_numbering: Query<&mut Visibility, With<DroneNumbering>>,
) {
    if drone_numbering_state.is_changed() {
        drone_numbering.iter_mut().for_each(|mut visibility| {
            if drone_numbering_state.show_drone_numbering {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        });
    }
}
