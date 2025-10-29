use crate::cityblock::nuclearplant::plant_status::PlantStatus;
use crate::cityblock::nuclearplant::supply_spec::SupplySpec;

pub struct CargoTruck {
    pub(crate) cargo: SupplySpec,
}

impl CargoTruck {
    pub fn new(cargo: SupplySpec, deadline_ms: usize) -> Self {
        Self {
            cargo,
        }
    }
    pub fn call(plant_status: PlantStatus) {

    }
}