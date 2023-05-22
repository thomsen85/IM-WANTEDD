use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

use super::components::OrbitCamera;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-20.0, 20.5, 50.0)
                .looking_at(Vec3::new(0.0, 0.0, 20.0), Vec3::Y),
            ..default()
        },
        OrbitCamera {
            radius: 50.0,
            rotation_offset: Quat::from_xyzw(1., 0., 0., 1.),
            upside_down: false,
            target: None,
        },
    ));
}

/// Makes the camera follow the drones.
pub fn camera_follow_entity(
    mut query: Query<(Entity, &Transform), Without<OrbitCamera>>,
    mut camera_query: Query<(&OrbitCamera, &mut Transform)>,
) {
    let mut camera = camera_query.single_mut();
    let center = match camera.0.target {
        Some(target) => {
            if let Ok((_, target_transform)) = query.get_mut(target) {
                target_transform.translation
            } else {
                Vec3::ZERO
            }
        }
        None => Vec3::ZERO,
    };

    let translation = camera
        .0
        .rotation_offset
        .mul_vec3(Vec3::new(0.0, camera.0.radius, 0.0));
    camera.1.translation = center + translation;
    camera.1.look_at(center, Vec3::Y);
}

pub fn orbit_camera(
    window: Query<&Window, With<PrimaryWindow>>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<&mut OrbitCamera>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Left;
    let primary_window = window.get_single().unwrap();

    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for mut orbit in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            // let up = orbit.rotation_offset * Vec3::Y;
            // orbit.upside_down = up.y <= 0.0;
        }

        if rotation_move.length_squared() > 0.0 {
            let delta_x = {
                let delta = rotation_move.x / primary_window.width() * std::f32::consts::PI * 2.0;
                if orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / primary_window.height() * std::f32::consts::PI;
            let sensitivity = 0.20; // Adjust this value to control the rotation speed
            let yaw_rotation = Quat::from_axis_angle(Vec3::Z, delta_x * sensitivity);
            let pitch_rotation = Quat::from_axis_angle(Vec3::NEG_X, delta_y * sensitivity);

            // Combine the rotations
            let rotation = yaw_rotation * pitch_rotation;

            orbit.rotation_offset *= rotation;
            orbit.rotation_offset = orbit.rotation_offset.normalize();
        } else if scroll.abs() > 0.0 {
            orbit.radius -= scroll * orbit.radius * 0.02;
            // dont allow zoom to reach zero or you get stuck
            orbit.radius = f32::max(orbit.radius, 0.05);
        }
    }

    // consume any remaining events, so they don't pile up if we don't need them
    // (and also to avoid Bevy warning us about not checking events every frame update)
    ev_motion.clear();
}
