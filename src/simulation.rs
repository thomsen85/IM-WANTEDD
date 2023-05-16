use std::sync::{Arc, RwLock};

pub struct World {
    pub drones: Vec<Drone>,
    pub emergency_beacons: Vec<EmergencyBeacon>,
}

pub struct Connection {}

pub struct Drone {
    world: Arc<RwLock<World>>,
    conntections: Vec<Connection>,
}

pub struct EmergencyBeacon {}
