use std::collections::HashMap;
use rand::prelude::IndexedRandom;
use mypthreads::mythread::mythread::ThreadId;
use crate::cityblock::block_type::BlockType::{Bridge, Dock, Road, Shops, Water, NuclearPlant};
use crate::cityblock::bridge::BridgeBlock;
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::cityblock::road::RoadBlock;
use crate::cityblock::water::WaterBlock;
use crate::vehicle::car::Car;
use crate::vehicle::vehicle::{MoveIntent, PatienceLevel, Vehicle};
use crate::vehicle::vehicle::PatienceLevel::{Maxed, Low, Critical, Starved};
use crate::vehicle::vehicle_type::VehicleType;
use crate::vehicle::vehicle_type::VehicleType::{ShipE};

pub struct TrafficHandler<'a> {
    pub(crate) vehicles: HashMap<ThreadId, Box<dyn Vehicle>>,
    road_coords: Vec<Coord>,
    shops_coords: Vec<Coord>,
    water_spawns: Vec<Coord>,
    dock: Option<Coord>,
    pub(crate) map: &'a mut Map,
    pub(crate) passed_frames: usize,
}

impl<'a> TrafficHandler<'a> {
    pub fn new(map: &'a mut Map, water_spawns: Vec<Coord>) -> Self {
        let dock = map.find_blocks(Dock).get(0).cloned(); // <- sin unwrap
        Self {
            vehicles: HashMap::new(),
            road_coords: map.find_blocks(Road),
            shops_coords: map.find_blocks(Shops),
            water_spawns,
            dock,
            map,
            passed_frames: 0,
        }
    }

    pub fn new_car(&mut self, tid: ThreadId) {
        let mut origin = Self::any_coord(self.road_coords.clone());
        let mut rb = self.map.get_block_at(origin);
        let mut road: &mut RoadBlock = rb.unwrap().as_any().downcast_mut::<RoadBlock>().unwrap();
        loop {
            if road.is_available() {
                road.consume_space();
                break;
            }
            origin = Self::any_coord(self.road_coords.clone());
            rb = self.map.get_block_at(origin);
            road = rb.unwrap().as_any().downcast_mut::<RoadBlock>().unwrap();
        }

        let destination = Self::any_coord(self.shops_coords.clone());
        let mut nc = Car::new(origin, destination);
        nc.initialize(self.map, tid);

        self.vehicles.insert(tid, Box::new(nc));
    }

    fn any_coord(vec: Vec<Coord>) -> Coord {
        vec.choose(&mut rand::rng()).cloned().unwrap()
    }

    /// El “reloj” global: avanza dt_ms y decide quién puede dar un paso.
    pub fn advance_time(&mut self) {
        for tid in self.vehicles.keys() {
            // Necesitamos sólo un *préstamo inmutable* del vehículo para calcular step_time e intención
            let Some(vref) = self.vehicles.get(tid) else { continue; };
            let vtype = vref.get_type();
            match vtype {
                ShipE => self.aquatic_intention(),
                _ => self.road_intention(tid, vref)
            }
        }

        // ---------- FASE 2: COMMIT ----------
        for dec in plan {
            match dec {
                Decision::MoveTo { tid, step_ms, from, to, is_bridge } => {
                    // (i) Si es puente, aquí invocarías a tu BridgeController (request_entry/exit)
                    // Por ahora, si 'is_bridge' prefieres tratarlo normal, continúa.
                    // Si no, integra:
                    //   if is_bridge { if !bridge_request_ok(...) { espera } }

                    // (ii) Chequeo de ocupación: ¡ya NO hay préstamo a v activo!
                    if self.occupancy.contains_key(&to) {
                        if let Some(acc) = self.time_acc.get_mut(&tid) {
                            *acc = step_ms.min(*acc); // ocupado → esperar
                        }
                        continue;
                    }

                    // (iii) Ocupación: liberar origen y ocupar destino
                    self.occupancy.remove(&from);
                    self.occupancy.insert(to, tid);

                    // (iv) Commit en el vehículo (ahora sí pedimos &mut al vehículo)
                    if let Some(vmut) = self.vehicles.get_mut(&tid) {
                        vmut.base_mut().commit_advance(to);
                    }

                    // (v) Consumir tiempo del paso
                    if let Some(acc) = self.time_acc.get_mut(&tid) {
                        *acc -= step_ms;
                    }

                    // (vi) Si saliste de un puente (from era puente y to no), libera el mutex del puente aquí
                    // if is_bridge_exit(from, to) { bridge_exit_with(tid); }
                }
            }
        }
    }


    fn road_intention(&mut self, tid: &ThreadId, vref: &Box<dyn Vehicle>) {
        // Consultamos intención SIN mutar el vehículo
        let intent = vref.plan_next_move(self.map);
        let from = vref.current();
        let current_block = self.map.get_block_at(from).unwrap();

        match intent {
            MoveIntent::Arrived => {
                self.vehicles.remove(&tid);
            }
            MoveIntent::AdvanceTo { coord: to } => {
                let mut next_block = self.map.get_block_at(to).unwrap();
                let mut next_rbl: &mut RoadBlock = next_block.as_any().downcast_mut::<RoadBlock>().unwrap();
                if let Some(vmut) = self.vehicles.get_mut(&tid) {
                    if let Some(bridge) = current_block.as_any().downcast_mut::<BridgeBlock>() {
                        self.handle_bridge_exit(next_rbl, vmut, bridge);
                        return;
                    }

                    let result = vmut.try_move(next_rbl.consume_space());
                    match result {
                         Maxed { moved } => {
                             if moved {
                                 let mut current_rbl = current_block.as_any().downcast_mut::<RoadBlock>().unwrap();
                                current_rbl.liberate_space();
                             }
                         }
                        Starved => {
                            self.vehicles.remove(&tid);
                        }
                        _ => {}
                    }
                }
            }
            MoveIntent::NextIsBridge { coord: to } => {
                plan.push(Decision::MoveTo {
                    tid: *tid, step_ms, from, to, is_bridge: true
                });
            }
            _ => {}
        }
    }
    fn aquatic_intention(&mut self) {}

    fn handle_bridge_exit(&mut self, to: &mut RoadBlock, vref: &mut Box<dyn Vehicle>, bridge: &mut BridgeBlock) {
        if to.is_available() {
            if (bridge.exit_bridge(vref)) {
                vref.try_move(to.consume_space());
            }
        }
    }
}


    fn any_coord(vec: Vec<Coord>) -> Coord {
        vec.choose(&mut rand::rng()).cloned().unwrap()
    }
