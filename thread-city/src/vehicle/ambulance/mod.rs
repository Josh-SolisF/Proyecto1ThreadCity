use std::any::Any;
use mypthreads::mythread::mythread::ThreadId;
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::vehicle::vehicle::{Vehicle, VehicleBase};
use crate::vehicle::vehicle_type::VehicleType;
use crate::vehicle::vehicle_type::VehicleType::AmbulanceE;

pub struct Ambulance {
    pub(crate) base: VehicleBase,
}

impl Ambulance {
    pub fn new(origin: Coord, destination: Coord, speed: u8) -> Ambulance {
        Self {
            base: VehicleBase::new(origin, destination, speed, AmbulanceE),
        }
    }
}

impl Vehicle for Ambulance {
    fn get_type(&self) -> &VehicleType {
        &self.base.vehicle_type
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn initialize(&mut self, map: &Map, thread_id: ThreadId) {
        self.base.calculate_path(map);
        self.base.thread_id = Some(thread_id);
    }
}