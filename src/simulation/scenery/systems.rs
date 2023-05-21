use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;

use super::{
    components::Ground,
    constants::{GROUND_HEIGHT, SHADOW_DISTANCE},
    resources::GroundState,
};
use std::f32::consts::PI;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("gjerdrum.gltf#Scene0"),
            transform: Transform::from_xyz(650.0, GROUND_HEIGHT, 400.0),
            ..default()
        },
        Ground {},
    ));
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
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

pub fn update_ground_height(
    state: Res<GroundState>,
    mut query: Query<&mut Transform, With<Ground>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.y = state.ground_height;
    }
}
