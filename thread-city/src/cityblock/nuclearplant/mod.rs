use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use mypthreads::mythread::mypthread::MyPThread;
use crate::city::supply_kind::SupplyKind;
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
    pub(crate) scheduled_kinds: Vec<SupplyKind>,

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
            scheduled_kinds: Vec::new(),
        }
    }



    pub fn advance_time(&mut self, time_passed: usize) -> Option<SupplySpec> {
        if self.plant_status == Boom { return None; }

        self.time_passed_ms += time_passed;

        if self.time_passed_ms >= self.update_interval_ms {
            self.time_passed_ms -= self.update_interval_ms;

            // Avanza un estado
            let prev = self.plant_status;
            self.plant_status = self.next_status();

            if self.plant_status == Boom {
                self.requires.clear();
                self.scheduled_kinds.clear();
                return None;
            }

            // Al entrar a AtRisk, crea los pedidos si aún no existen
            if prev != AtRisk && self.plant_status == AtRisk && self.requires.is_empty() {
                self.enqueue_default_requirements();
            }
        }

        None
    }



    fn enqueue_default_requirements(&mut self) {
        use crate::city::supply_kind::SupplyKind;

        let dl = self.dead_line_policy;
        self.requires.clear();
        self.requires.push(SupplySpec { kind: SupplyKind::NuclearMaterial, dead_line: dl, time_passed_ms: 0 });
        self.requires.push(SupplySpec { kind: SupplyKind::Water,            dead_line: dl, time_passed_ms: 0 });
    }


    pub fn spawn_trucks_for_pending_requirements<F>(
        &mut self,
        plant_coord: Coord,
        mut pick_origin: F,
    ) -> Vec<CargoTruck>
    where
        F: FnMut(&SupplySpec) -> Coord,
    {
        if matches!(self.plant_status, Ok | Boom) {
            return Vec::new();
        }

        let mut created = Vec::new();

        for req in self.requires.iter() {
            // ¿ya hay uno programado de este tipo?
            if self.scheduled_kinds.iter().any(|k| *k == req.kind) {
                continue;
            }

            let origin = pick_origin(req);
            let speed = 1; // Ajusta si manejas velocidades
            let truck = CargoTruck::new(origin, plant_coord, speed, *req);

            // Marca como programado por tipo:
            self.scheduled_kinds.push(req.kind);

            // Para que lo registres en el runtime/mapa:
            created.push(truck);
        }

        created
    }
    pub fn mark_truck_scheduled_for_kind(&mut self, kind: SupplyKind) {
        if !self.scheduled_kinds.iter().any(|k| *k == kind) {
            self.scheduled_kinds.push(kind);
        }

    }
    /// Interno: avanza el estado UNA vez y, si entra a AtRisk, genera pedidos.
    /// Devuelve un pedido pendiente sin camión programado (si lo hay) para que lo atiendas ya.
    fn check_requirements(&mut self) -> Option<SupplySpec> {
        use crate::city::supply_kind::SupplyKind;

        let prev = self.plant_status;
        self.plant_status = self.next_status();

        if self.plant_status == Boom {
            // Limpieza final opcional
            self.requires.clear();
            self.scheduled_kinds.clear();
            return None;
        }

        // Al ENTRAR a AtRisk por primera vez, crea pedidos si no existen.
        if prev != AtRisk && self.plant_status == AtRisk && self.requires.is_empty() {
            let dl = self.dead_line_policy;
            self.requires.push(SupplySpec { kind: SupplyKind::NuclearMaterial, dead_line: dl, time_passed_ms: 0 });
            self.requires.push(SupplySpec { kind: SupplyKind::Water,            dead_line: dl, time_passed_ms: 0 });
        }

        // En AtRisk o Critical, si hay requerimientos pendientes NO programados, pídelo.
        if matches!(self.plant_status, AtRisk | Critical) {
            return self.next_outstanding_to_schedule();
        }

        None
    }


    pub fn commit_delivery(&mut self, truck: &CargoTruck) {
        let delivered_kind = truck.cargo.kind;

        //  Elimina el requerimiento por tipo
        let before = self.requires.len();
        self.requires.retain(|req| req.kind != delivered_kind);

        // Deja de considerarlo programado
        self.scheduled_kinds.retain(|k| *k != delivered_kind);

        // Si ya NO hay requerimientos pendientes, sube un nivel (máximo uno)
        if before > 0 && self.requires.is_empty() {
            self.plant_status = match self.plant_status {
                AtRisk   => Ok,
                Critical => AtRisk,
                other    => other, // Ok o Boom no deberían llegar aquí, pero por seguridad.
            };
        }
    }


fn next_status(&mut self) -> PlantStatus {
    match self.plant_status {
        Ok       => AtRisk,
        AtRisk   => Critical,
        Critical => {
            // Al entrar a Boom, limpia los internos
            self.dead_line_policy = 0;
            self.update_interval_ms = 0;
            self.requires.clear();
            self.scheduled_kinds.clear();
            Boom
        }
        Boom     => Boom,
    }
}
}
/*

    /// Interno: siguiente estado en la cadena Ok -> AtRisk -> Critical -> Boom
    fn next_status(&mut self) -> PlantStatus {
        match self.plant_status {
            Ok       => AtRisk,
            AtRisk   => Critical,
            Critical => {
                // Llegar a Boom: deja la planta inoperante. Limpia políticas si quieres
                self.dead_line_policy = 0;
                self.update_interval_ms = 0;
                Boom
            }
            Boom     => Boom,
        }
    }

    /// Interno: devuelve el siguiente SupplySpec pendiente que aún NO tenga camión programado.
    fn next_outstanding_to_schedule(&self) -> Option<SupplySpec> {
        self.requires
            .iter()
            .find(|req| !self.is_kind_scheduled(req.kind))
            .copied()
    }

    /// Interno: ¿ya hay camión programado para este tipo?
    fn is_kind_scheduled(&self, kind: crate::city::supply_kind::SupplyKind) -> bool {
        self.scheduled_trucks.iter().any(|t| t.cargo.kind == kind)
    }
        pub fn commit_delivery(&mut self, truck: &CargoTruck) {
            let delivered_kind = truck.cargo.kind;

            // Elimina el requerimiento por ese tipo
            let before = self.requires.len();
            self.requires.retain(|req| req.kind != delivered_kind);

            // También deja de considerarlo "programado"
            self.scheduled_kinds.retain(|k| *k != delivered_kind);

            // Si ya no hay requerimientos pendientes, sube un estado (máximo un nivel)
            if before > 0 && self.requires.is_empty() {
                self.plant_status = match self.plant_status {
                    AtRisk   => Ok,
                    Critical => AtRisk,
                    other    => other,
                };
            }
        }

}
*/
