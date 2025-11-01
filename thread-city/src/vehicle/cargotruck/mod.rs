use std::any::Any;
use mypthreads::mythread::mythread::ThreadId;
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::cityblock::nuclearplant::plant_status::PlantStatus;
use crate::cityblock::nuclearplant::supply_spec::SupplySpec;
use crate::vehicle::vehicle::{Vehicle, VehicleBase};
use crate::vehicle::vehicle_type::VehicleType;
use crate::vehicle::vehicle_type::VehicleType::TruckE;

pub struct CargoTruck {
    pub(crate) cargo: SupplySpec,
    pub(crate) base: VehicleBase,
}

impl CargoTruck {
    pub fn new(origin: Coord, destination: Coord, speed: u8, cargo: SupplySpec) -> Self {
        Self {
            cargo,
            base: VehicleBase::new(origin, destination, speed, TruckE),
        }
    }
    pub fn call(plant_status: PlantStatus) {

    }
}

impl Vehicle for CargoTruck {
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