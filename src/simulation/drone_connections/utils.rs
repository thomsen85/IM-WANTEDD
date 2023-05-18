use crate::simulation::drones::components::Drone;
use bevy::prelude::*;

pub fn get_nearby_drones<'a>(
    current_drone: (&Drone, &Transform),
    drones: &Vec<(&'a Drone, &'a Transform)>,
    distance: f32,
) -> Vec<(&'a Drone, &'a Transform)> {
    let mut nearby_drones = Vec::new();
    for (drone, transform) in drones.iter() {
        if transform.translation.distance(current_drone.1.translation) < distance {
            nearby_drones.push((*drone, *transform));
        }
    }
    nearby_drones
}
