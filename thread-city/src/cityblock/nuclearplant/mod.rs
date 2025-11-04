use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use mypthreads::mythread::mypthread::MyPThread;
use crate::cityblock::map::Map;
use crate::cityblock::{Block, BlockBase};
use crate::cityblock::block_type::BlockType;
use crate::cityblock::block_type::BlockType::NuclearPlant;
use crate::cityblock::coord::Coord;
use crate::cityblock::nuclearplant::plant_status::PlantStatus;
use crate::cityblock::nuclearplant::plant_status::PlantStatus::{Ok, AtRisk, Critical, Boom};
use crate::cityblock::nuclearplant::supply_spec::SupplySpec;
use crate::cityblock::transport_policy::TransportPolicy;
use crate::cityblock::transport_policy::TransportPolicy::NoVehicles;
use crate::vehicle::cargotruck::CargoTruck;

pub mod plant_status;
pub mod supply_spec;

pub struct NuclearPlantBlock {
    pub(crate) base: BlockBase,
    pub(crate) plant_status: PlantStatus,
    pub(crate) time_passed_ms: usize,
    pub(crate) dead_line_policy: usize,
    pub(crate) update_interval_ms: usize,
    pub(crate) requires: Vec<SupplySpec>,
    pub(crate) scheduled_trucks: Vec<CargoTruck>,
}

impl Block for NuclearPlantBlock {
    fn get_id(&self) -> &usize {
        &self.base.id
    }
    fn get_policy(&self) -> &TransportPolicy {
        &self.base.policy
    }
    fn get_type(&self) -> &BlockType {
        &self.base.block_type
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl NuclearPlantBlock {
    pub fn new(id: usize, dead_line_policy: usize, update_interval_ms: usize) -> Self {

         Self {
            base: BlockBase::new(id, NoVehicles, NuclearPlant),
            plant_status: Ok,
            dead_line_policy,
            requires: Vec::new(),
            time_passed_ms: 0,
            update_interval_ms,
            scheduled_trucks: Vec::new(),
        }
    }

    /// #Return
    /// 'None' in case it doesn't require a cargotruck
    /// 'Some(SupplySpec)' in case it needs a cargotruck to be sent
    pub fn advance_time(&mut self, time_passed: usize) -> Option<SupplySpec> {
        if self.plant_status == Boom { return None; }
        self.time_passed_ms += time_passed;
        if self.time_passed_ms >= self.update_interval_ms {
            self.time_passed_ms -= self.update_interval_ms;
            self.check_requirements();
        }
        None
    }

    pub fn add_truck_status(&mut self, truck: CargoTruck) {
        self.scheduled_trucks.push(truck);
    }

    pub fn commit_delivery(&mut self, truck: &CargoTruck) {
        if (self.requires.contains(&truck.cargo)) {
            todo!("Deberia eliminarse el camión y el supply que esperaba la planta")
        }
        
        if self.requires.is_empty() {
            self.plant_status = Ok;
        }
    }

    fn check_requirements(&mut self) -> Option<SupplySpec> {
        let next = self.next_status();
        self.plant_status = next;

        if next == Boom { return None}

        if next == AtRisk {
            todo!("Generar 1 o 2 camiones con requerimientos de la planta, usando el mapa claro esta")
        }
        if next == Critical {
            todo!("Darle el máximo de prioridad a los camiones")
        }

        None

    }
    fn next_status(&mut self) -> PlantStatus {
        match self.plant_status {
            Ok => {AtRisk}
            AtRisk => {Critical},
            Critical => {
                self.dead_line_policy = 0;
                self.requires = Vec::new();
                self.update_interval_ms = 0;
                Boom
            }
            Boom => { Boom }
        }
    }

}