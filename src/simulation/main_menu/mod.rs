use bevy::prelude::*;

use self::components::OnMainMenuScreen;

use super::AppState;

pub struct MainMenuPlugin;

pub mod components;
mod constants;
mod systems;

/// This plugin is greatly inspired by the UI example in the Bevy repo:
/// https://github.com/bevyengine/bevy/blob/release-0.10.1/examples/games/game_menu.rs
///
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(systems::setup.in_schedule(OnEnter(AppState::Menu)))
            .add_systems(
                (systems::button_system, systems::menu_action).in_set(OnUpdate(AppState::Menu)),
            )
            .add_system(
                systems::despawn_screen::<OnMainMenuScreen>.in_schedule(OnExit(AppState::Menu)),
            );
    }
}
