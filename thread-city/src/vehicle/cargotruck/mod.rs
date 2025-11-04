use std::any::Any;
use mypthreads::mythread::mythread::ThreadId;
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::cityblock::nuclearplant::plant_status::PlantStatus;
use crate::cityblock::nuclearplant::supply_spec::SupplySpec;
use crate::vehicle::vehicle::{MoveIntent, PatienceLevel, Vehicle, VehicleBase};
use crate::vehicle::vehicle::PatienceLevel::Maxed;
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
            base: VehicleBase::new(origin, destination, TruckE, 3),
        }
    }
    pub fn call(&mut self, plant_status: PlantStatus) {
        match plant_status {
            PlantStatus::Ok => { self.base.patience = self.base.max_patience }
            PlantStatus::AtRisk => { self.base.patience = 2 }
            PlantStatus::Critical => { self.base.patience = 1 }
            PlantStatus::Boom => { self.base.patience = 0 }
        }
    }
}

impl Vehicle for CargoTruck {
    fn get_type(&self) -> &VehicleType {
        &self.base.vehicle_type
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn initialize(&mut self, map: &Map, thread_id: ThreadId) {
        self.base.calculate_path(map);
        self.base.thread_id = Some(thread_id);
    }

    fn plan_next_move(&self, map: &Map) -> MoveIntent {
        if self.base.current_position == self.base.destination ||
            self.base.path_idx >= (self.base.path.as_ref().unwrap().len() - 1) {
            return MoveIntent::Arrived;
        }
        self.base.plan_next(map)
    }

    fn try_move(&mut self, next_is_open: bool) -> PatienceLevel {
        if next_is_open {
            self.base.patience = self.base.max_patience;
            self.base.current_position = self.base.path.as_mut().unwrap()[self.base.path_idx];
            self.base.path_idx += 1;
            return Maxed {moved: true};
        }
        self.base.patience -= 1;
        self.calc_patience()
    }

    fn base(&self) -> &VehicleBase { &self.base }
    fn base_mut(&mut self) -> &mut VehicleBase { &mut self.base }

    fn calc_patience(&self) -> PatienceLevel {
        todo!()
    }
}