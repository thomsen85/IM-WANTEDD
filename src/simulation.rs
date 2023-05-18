mod camera;
mod drone_connections;
mod drones;
mod emergency_beacons;
mod ui;

use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

const GROUND_HEIGHT: f32 = -200.0;
const SHADOW_DISTANCE: f32 = 200.0;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ui::UIPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(drones::DronesPlugin)
        .add_plugin(drone_connections::DroneConnectionsPlugin)
        .add_startup_system(setup)
        .add_system(update_ground_height)
        .init_resource::<GroundState>()
        .run();
}

#[derive(Resource)]
pub struct GroundState {
    pub ground_height: f32,
}

impl Default for GroundState {
    fn default() -> Self {
        Self {
            ground_height: GROUND_HEIGHT,
        }
    }
}

#[derive(Component)]
pub struct Ground;

fn update_ground_height(state: Res<GroundState>, mut query: Query<&mut Transform, With<Ground>>) {
    for mut transform in query.iter_mut() {
        transform.translation.y = state.ground_height;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("gjerdrum.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, GROUND_HEIGHT, 0.0),
            ..default()
        },
        Ground {},
    ));
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: SHADOW_DISTANCE,
            ..default()
        }
        .into(),
        ..default()
    });
}
