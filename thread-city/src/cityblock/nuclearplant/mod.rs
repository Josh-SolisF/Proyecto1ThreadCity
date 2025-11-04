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

    /// Avanza el "reloj". Solo decide si hay transición y la aplica.
    /// Devuelve siempre None (firma preservada para compatibilidad).
    pub fn advance_time(&mut self, time_passed: usize) -> Option<SupplySpec> {
        if self.plant_status == Boom { return None; }

        self.time_passed_ms += time_passed;


        if self.time_passed_ms >= self.update_interval_ms {
            self.time_passed_ms -= self.update_interval_ms;

            let next = self.compute_next_status();
            self.apply_transition(next);
        }

        None
    }

    /// Pone los 2 requerimientos por defecto al entrar en AtRisk.
    fn enqueue_default_requirements(&mut self) {
        let dl = self.dead_line_policy;
        self.requires.clear();
        self.requires.push(SupplySpec { kind: SupplyKind::NuclearMaterial, dead_line: dl, time_passed_ms: 0 });
        self.requires.push(SupplySpec { kind: SupplyKind::Water,            dead_line: dl, time_passed_ms: 0 });
    }

    /// La planta crea camiones para cada requerimiento pendiente sin camión programado.
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
            if self.is_kind_scheduled(req.kind) {
                continue;
            }

            let origin = pick_origin(req);
            let speed = 1;
            // Nota: si SupplySpec no es Copy, usa `req.clone()`.
            let truck = CargoTruck::new(origin, plant_coord, speed, *req);

            // Marca como programado (por tipo) y lo retorna para que tu runtime lo registre.
            self.scheduled_kinds.push(req.kind);
            created.push(truck);
        }

        created
    }

    pub fn mark_truck_scheduled_for_kind(&mut self, kind: SupplyKind) {
        if !self.is_kind_scheduled(kind) {
            self.scheduled_kinds.push(kind);
        }
    }

    /// Llamar cuando un camión llega y entrega.
    /// Si cumple todos los pedidos pendientes, la planta sube un nivel (máximo uno).
    pub fn commit_delivery(&mut self, truck: &CargoTruck) {
        let delivered_kind = truck.cargo.kind;

        let before = self.requires.len();
        self.requires.retain(|req| req.kind != delivered_kind);
        self.scheduled_kinds.retain(|k| *k != delivered_kind);

        if before > 0 && self.requires.is_empty() {
            self.plant_status = match self.plant_status {
                AtRisk   => Ok,
                Critical => AtRisk,
                other    => other, // seguridad
            };
        }
    }


    fn compute_next_status(&self) -> PlantStatus {
        match self.plant_status {
            Ok       => AtRisk,
            AtRisk   => Critical,
            Critical => Boom,
            Boom     => Boom,
        }
    }

    fn apply_transition(&mut self, next: PlantStatus) {
        let prev = self.plant_status;
        if prev == next { return; }

        self.plant_status = next;

        match next {
            AtRisk => {
                // Solo creamos pedidos al entrar a AtRisk si aún no hay
                if self.requires.is_empty() {
                    self.enqueue_default_requirements();
                }
            }
            Critical => {
                //
            }
            Boom => {
                // kaboom
                self.requires.clear();
                self.scheduled_kinds.clear();
                self.dead_line_policy = 0;
                self.update_interval_ms = 0;
            }
            Ok => {

            }
        }
    }

    /// ¿Ya hay camión programado para `kind`?
    fn is_kind_scheduled(&self, kind: SupplyKind) -> bool {
        self.scheduled_kinds.iter().any(|k| *k == kind)
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
*/



