use bevy::prelude::*;

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct SelectedOption;

#[derive(Component, Clone, Debug)]
pub enum ScenarioChosen {
    SingleBeacon,
    MultipleBeacons,
    UnstableConnections,
    MeetingDroneMeshes,
}
