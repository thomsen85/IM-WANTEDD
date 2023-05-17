mod FpsCounter;
use crate::simulation::CameraState;
use crate::simulation::ui::FpsCounter::{Fps, FpsCounterPlugin};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};


pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ui_system)
        .add_plugin(FpsCounterPlugin)
        ;
    }
}

fn ui_system(
  mut contexts: EguiContexts,
  fps: Res<Fps>,
  mut camera_state: ResMut<CameraState>,
) {
  egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
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
      });
  });
}