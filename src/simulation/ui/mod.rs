mod fps_counter;

use self::fps_counter::{FpsCounterPlugin, resources::Fps};

use super::{drones::Drone, camera::resources::CameraState, scenery::resources::GroundState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(FpsCounterPlugin)
            .add_system(controll_ui)
            .add_system(console_ui)
            .add_system(drone_numbering)
            .init_resource::<DroneNumberingState>()
            .add_system(drone_numbering_toggle);
    }
}

#[derive(Resource, Default)]
pub struct DroneNumberingState {
    pub show_drone_numbering: bool,
}

fn controll_ui(
    mut contexts: EguiContexts,
    fps: Res<Fps>,
    mut camera_state: ResMut<CameraState>,
    mut ground_state: ResMut<GroundState>,
    mut drone_numbering_state: ResMut<DroneNumberingState>,
) {
    egui::Window::new("Controlls").show(contexts.ctx_mut(), |ui| {
        egui::Grid::new("My_Grid").num_columns(2).show(ui, |ui| {
            ui.label("FPS:");
            ui.label(format!("{:.2}", fps.amount));
            ui.end_row();
            ui.label("Camera pos x:");
            ui.add(egui::DragValue::new(&mut camera_state.relative_pos.x).speed(0.1));
            ui.end_row();
            ui.label("Camera pos y:");
            ui.add(egui::DragValue::new(&mut camera_state.relative_pos.y).speed(0.1));
            ui.end_row();
            ui.label("Camera pos z:");
            ui.add(egui::DragValue::new(&mut camera_state.relative_pos.z).speed(0.1));
            ui.end_row();
            ui.label("Ground pos:");
            ui.add(egui::DragValue::new(&mut ground_state.ground_height).speed(0.1));
            ui.end_row();
            ui.add(egui::Checkbox::new(
                &mut drone_numbering_state.show_drone_numbering,
                "Show Drone Numbering",
            ));
        });
    });
}

fn console_ui(mut contexts: EguiContexts) {
    egui::Window::new("Console").show(contexts.ctx_mut(), |ui| {
        ui.label("Hello World!");
    });
}

#[derive(Component)]
pub struct DroneNumbering {
    pub drone: Entity,
}

fn drone_numbering(
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
            println!("Spawned at");
        }
    }
}

fn drone_numbering_toggle(
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
