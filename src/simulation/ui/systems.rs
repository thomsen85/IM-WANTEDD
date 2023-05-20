use crate::simulation::drones::resources::DroneState;
use crate::simulation::emergency_pings::resources::EmergencyPingState;
use crate::simulation::scenery::resources::GroundState;
use crate::simulation::ui::drone_numbering::resources::DroneNumberingState;
use crate::simulation::ui::fps_counter::resources::Fps;
use crate::simulation::{camera::resources::CameraState, drones::components::Drone};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui_extras::{Column, TableBuilder};

use super::resources::UiState;

pub fn controll_ui(
    mut contexts: EguiContexts,
    fps: Res<Fps>,
    mut camera_state: ResMut<CameraState>,
    mut ground_state: ResMut<GroundState>,
    mut drone_numbering_state: ResMut<DroneNumberingState>,
    mut emergency_ping_state: ResMut<EmergencyPingState>,
    mut drone_state: ResMut<DroneState>,
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
            ui.end_row();
            ui.add(egui::Checkbox::new(
                &mut emergency_ping_state.visible,
                "Show Emergency Pings",
            ));
            ui.end_row();
            ui.add(egui::DragValue::new(&mut drone_state.drone_connection_range).speed(0.1));
        });
    });
}

pub fn console_ui(
    mut contexts: EguiContexts,
    drones: Query<(Entity, &Drone)>,
    mut ui_state: ResMut<UiState>,
) {
    egui::Window::new("Drones").show(contexts.ctx_mut(), |ui| {
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("ID");
                });
                header.col(|ui| {
                    ui.strong("Position");
                });
                header.col(|ui| {
                    ui.strong("Velocity");
                });
            })
            .body(|mut body| {
                for (drone_entity_id, drone) in drones.iter() {
                    body.row(18.0, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{}", drone.id));
                        });
                        row.col(|ui| {
                            ui.label("Hello");
                        });
                        row.col(|ui| {
                            let button = ui.button("Show Data");
                            if button.clicked() {
                                ui_state.current_drone = Some(drone_entity_id);
                            }
                        });
                    });
                }
            });
    });
}

pub fn drone_detail_ui(
    mut contexts: EguiContexts,
    drones: Query<(Entity, &Drone)>,
    ui_state: Res<UiState>,
) {
    if let Some(drone_entity_id) = ui_state.current_drone {
        if let Ok((_drone_entity_id, drone)) = drones.get(drone_entity_id) {
            egui::Window::new(format!("Drone {} Details", drone.id))
                .id(egui::Id::new("demo_window_options")) // required since we change the title
                .show(contexts.ctx_mut(), |ui| {
                    TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .header(18.0, |mut header| {
                            header.col(|ui| {
                                ui.strong("Timestamp");
                            });
                            header.col(|ui| {
                                ui.strong("Drone ID");
                            });
                            header.col(|ui| {
                                ui.strong("Emergency Beacon ID");
                            });
                            header.col(|ui| {
                                ui.strong("Distance");
                            });
                        })
                        .body(|mut body| {
                            let mut emergency_pings = drone.data.clone();
                            emergency_pings.sort_by_key(|ping| ping.timestamp);

                            for emergency_ping in emergency_pings {
                                body.row(18.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(format!(
                                            "{}",
                                            emergency_ping.timestamp.to_rfc2822()
                                        ));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{}", emergency_ping.drone_id));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{}", emergency_ping.emergency_beacon_id));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{}", emergency_ping.distance));
                                    });
                                });
                            }
                        });
                });
        }
    }
}
