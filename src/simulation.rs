pub struct World {
    pub drones: Vec<Drone>,
    pub emergency_beacons: Vec<EmergencyBeacon>,
}

pub struct Drone {
    world: Arc<RwLock<World>>,
    conntections: Vec<Conntection>,
}

pub struct Connection {}

pub struct EmergencyBeacon {}
