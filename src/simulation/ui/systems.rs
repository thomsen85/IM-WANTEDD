use crate::simulation::camera::resources::CameraState;
use crate::simulation::scenery::resources::GroundState;
use crate::simulation::ui::drone_numbering::resources::DroneNumberingState;
use crate::simulation::ui::fps_counter::resources::Fps;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn controll_ui(
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

pub fn console_ui(mut contexts: EguiContexts) {
    egui::Window::new("Console").show(contexts.ctx_mut(), |ui| {
        ui.label("Hello World!");
    });
}
