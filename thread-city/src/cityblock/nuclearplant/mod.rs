use mypthreads::mythread::mypthread::MyPThread;
use crate::city::map::Map;
use crate::cityblock::nuclearplant::plant_status::PlantStatus;
use crate::cityblock::nuclearplant::plant_status::PlantStatus::{Ok, AtRisk, Critical, Boom};
use crate::cityblock::nuclearplant::supply_spec::SupplySpec;
use crate::vehicle::cargotruck::CargoTruck;

pub mod plant_status;
pub mod supply_spec;

pub struct NuclearPlant {
    pub(crate) plant_status: PlantStatus,
    pub(crate) time_passed_ms: usize,
    pub(crate) dead_line_policy: usize,
    pub(crate) update_interval_ms: usize,
    pub(crate) requires: Vec<SupplySpec>,
    pub(crate) scheduled_trucks: Vec<CargoTruck>,
    pub(crate) city_map: *mut Map
}

impl NuclearPlant {
    pub fn new(dead_line_policy: usize, update_interval_ms: usize, city_map: *mut Map) -> Self {
        Self {
            plant_status: Ok,
            dead_line_policy,
            requires: Vec::new(),
            time_passed_ms: 0,
            update_interval_ms,
            scheduled_trucks: Vec::new(),
            city_map
        }
    }

    pub fn advance_time(&mut self, time_passed: usize) {
        if self.plant_status == Boom { return }
        self.time_passed_ms += time_passed;
        if self.time_passed_ms >= self.update_interval_ms {
            self.time_passed_ms -= self.update_interval_ms;
            self.check_requirements();
        }
    }

    pub fn commit_delivery(&mut self, truck: CargoTruck) {
        if (self.requires.contains(&truck.cargo)) {
            todo!("Deberia eliminarse el camión y el supply que esperaba la planta")
        }
        
        if self.requires.is_empty() {
            self.plant_status = Ok;
        }
    }

    fn check_requirements(&mut self) {
        let next = self.next_status();
        self.plant_status = next;

        if next == Boom { return }

        if next == AtRisk {
            todo!("Generar 1 o 2 camiones con requerimientos de la planta, usando el mapa claro esta")
        }
        if next == Critical {
            todo!("Darle el máximo de prioridad a los camiones")
        }

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