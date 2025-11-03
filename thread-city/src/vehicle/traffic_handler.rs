use std::collections::HashMap;
use rand::prelude::IndexedRandom;
use mypthreads::mythread::mythread::ThreadId;
use crate::cityblock::block_type::BlockType::{Dock, Road, Shops, Water};
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::vehicle::car::Car;
use crate::vehicle::vehicle::{MoveIntent, Occupancy, Vehicle};
use crate::cityblock::block_type::BlockType;
use crate::cityblock::bridge::BridgeBlock;
use crate::cityblock::bridge::BridgePermissionEnum::EntryOutcome;
pub struct TrafficHandler<'a> {
    pub(crate) vehicles: HashMap<ThreadId, Box<dyn Vehicle>>,
    road_coords: Vec<Coord>,
    shops_coords: Vec<Coord>,
    water_spawns: Vec<Coord>,
    dock: Option<Coord>,
    pub(crate) map: &'a Map,

    pub(crate) occupancy: HashMap<Coord, ThreadId>,
    pub(crate) time_acc: HashMap<ThreadId, f32>, // ms acumulados por vehículo



}

impl<'a> TrafficHandler<'a> {
    pub fn new(map: &'a Map, water_spawns: Vec<Coord>) -> Self {

        let dock = map.find_blocks(Dock).get(0).cloned(); // <- sin unwrap

        Self {
            vehicles: HashMap::new(),
            road_coords: map.find_blocks(Road),
            shops_coords: map.find_blocks(Shops),
            water_spawns,
            dock,
            //dock: map.find_blocks(Dock).get(0).unwrap().clone(),
            map,

            occupancy: HashMap::new(),
            time_acc: HashMap::new(),


        }



    }


    pub fn new_car(&mut self, tid: ThreadId) {
        let origin = Self::any_coord(self.road_coords.clone());
        let destination = Self::any_coord(self.shops_coords.clone());
        let mut nc = Car::new(origin, destination, 1);
        nc.initialize(self.map, tid);

        self.occupancy.insert(origin, tid);
        self.time_acc.insert(tid, 0.0);
        self.vehicles.insert(tid, Box::new(nc));
    }

    fn any_coord(vec: Vec<Coord>) -> Coord {
        vec.choose(&mut rand::rng()).cloned().unwrap()
    }

    #[inline]
    fn step_ms(speed_cps: f32) -> f32 {
        (1000.0_f32) / speed_cps.max(1.0)
    }

    /// El “reloj” global: avanza dt_ms y decide quién puede dar un paso.
    /// El “reloj” global: avanza dt_ms y decide quién puede dar un paso.
    pub fn tick(&mut self, dt_ms: u64) {


        let dt = dt_ms as f32;

        // 1) Snapshot de TIDs para evitar problemas de préstamo del HashMap
        let tids: Vec<ThreadId> = self.vehicles.keys().cloned().collect();

        // 2) Estructura con las decisiones planeadas (no aplicadas aún)
        #[derive(Debug)]
        enum Decision {
            Arrived { tid: ThreadId },
            Wait    { tid: ThreadId, step_ms: f32 },
            MoveTo  { tid: ThreadId, step_ms: f32, from: Coord, to: Coord, is_bridge: bool },
        }
        let mut plan: Vec<Decision> = Vec::with_capacity(tids.len());

        // ---------- FASE 1: PLAN ----------
        for tid in &tids {
            // Acumular tiempo para este vehículo
            let acc = self.time_acc.entry(*tid).or_insert(0.0);
            *acc += dt;

            // Necesitamos sólo un *préstamo inmutable* del vehículo para calcular step_time e intención
            let Some(vref) = self.vehicles.get(tid) else { continue; };
            let step_ms = Self::step_ms(vref.base().speed_cps());
            if *acc < step_ms {
                // Aún no le toca moverse -> nada que planear
                continue;
            }

            // Consultamos intención SIN mutar el vehículo
            let intent = vref.base().plan_next(self.map);
            let from = vref.base().current();

            match intent {
                MoveIntent::Arrived => {
                    plan.push(Decision::Arrived { tid: *tid });
                }
                MoveIntent::NoPath | MoveIntent::BlockedByPolicy => {
                    // No avanza, pero dejamos el acumulador cerca del umbral en COMMIT
                    plan.push(Decision::Wait { tid: *tid, step_ms });
                }
                MoveIntent::AdvanceTo { coord: to } => {
                    plan.push(Decision::MoveTo {
                        tid: *tid, step_ms, from, to, is_bridge: false
                    });
                }
                MoveIntent::NextIsBridge { coord: to } => {
                    plan.push(Decision::MoveTo {
                        tid: *tid, step_ms, from, to, is_bridge: true
                    });
                }
            }
        }

        // ---------- FASE 2: COMMIT ----------
        for dec in plan {
            match dec {
                Decision::Arrived { tid } => {
                    // a) liberar la celda ocupada (si quieres)
                    if let Some(v) = self.vehicles.get(&tid) {
                        let pos = v.base().current();
                        self.occupancy.remove(&pos);
                    }
                    // b) remover vehículo y su reloj
                    self.vehicles.remove(&tid);
                    self.time_acc.remove(&tid);
                    // c) métricas/estadísticas si aplica...
                }

                Decision::Wait { tid, step_ms } => {
                    // Lo dejas listo para reintentar pronto (pegado al umbral)
                    if let Some(acc) = self.time_acc.get_mut(&tid) {
                        *acc = step_ms.min(*acc);
                    }
                }

                Decision::MoveTo { tid, step_ms, from, to, is_bridge } => {
                    // Detecta relación con puente (evita helpers redundantes)
                    let from_is_bridge = matches!(self.map.block_type_at(from), Some(BlockType::Bridge));
                    let to_is_bridge   = is_bridge || matches!(self.map.block_type_at(to), Some(BlockType::Bridge));

                    let is_entering = !from_is_bridge && to_is_bridge;
                    let is_exiting  =  from_is_bridge && !to_is_bridge;
                    // mover dentro del puente: from_is_bridge && to_is_bridge (no tocar lock)

                    // (0) Si el destino está ocupado, esperar (no intentes reservar el carril)
                    if self.occupancy.contains_key(&to) {
                        if let Some(acc) = self.time_acc.get_mut(&tid) {
                            *acc = step_ms.min(*acc);
                        }
                        continue;
                    }

                    // (1) Si estás entrando a un puente, intenta pedir entrada (reserva carril si Granted)
                    if is_entering {
                        // Necesitamos el vehículo para validar policy/tipo
                        let Some(vref) = self.vehicles.get(&tid) else { continue; };

                        let can_enter = match self.map.block_at_mut(to) {
                            Some(b) => {
                                // Downcast seguro al BridgeBlock concreto
                                if let Some(bridge) = b.as_any_mut().downcast_mut::<BridgeBlock>() {
                                    match bridge.request_entry(vref.base(), from, to, tid) {
                                        EntryOutcome::Granted   => true,  // carril reservado
                                        EntryOutcome::Wait      => false, // rojo u ocupado
                                        EntryOutcome::Forbidden => false, // política lo prohíbe
                                    }
                                } else {
                                    // Si el tipo dice Bridge pero no es BridgeBlock, negar por seguridad
                                    false
                                }
                            }
                            None => false,
                        };

                        if !can_enter {
                            if let Some(acc) = self.time_acc.get_mut(&tid) {
                                *acc = step_ms.min(*acc);
                            }
                            continue;
                        }
                        // Si llegó aquí, el carril quedó reservado para 'tid' hasta que salga del puente.
                    }

                    // (2) Ocupación: liberar origen y ocupar destino
                    self.occupancy.remove(&from);
                    self.occupancy.insert(to, tid);

                    // (3) Commit en el vehículo
                    if let Some(vmut) = self.vehicles.get_mut(&tid) {
                        vmut.base_mut().commit_advance(to);
                    }

                    // (4) Consumir tiempo del paso
                    if let Some(acc) = self.time_acc.get_mut(&tid) {
                        *acc -= step_ms;
                    }

                    // (5) Si estás saliendo del puente, libera el carril después de moverte
                    if is_exiting {
                        if let (Some(b), Some(vref)) = (self.map.block_at_mut(from), self.vehicles.get(&tid)) {
                            if let Some(bridge) = b.as_any_mut().downcast_mut::<BridgeBlock>() {
                                let _ = bridge.exit_bridge(vref.base(), tid);
                            }
                        }
                    }
                }
            }
        }

        // ---------- INFRA: avanzar semáforos de puentes ----------
        // Si tienes un índice de puentes, úsalo en vez de recorrer toda la grilla.
        for y in 0..self.map.height() {
            for x in 0..self.map.width() {
                let c = Coord { x: x as i32, y: y as i32 };
                if matches!(self.map.block_type_at(c), Some(BlockType::Bridge)) {
                    if let Some(b) = self.map.block_at_mut(c) {
                        if let Some(bridge) = b.as_any_mut().downcast_mut::<BridgeBlock>() {
                            bridge.advance_time(dt_ms as usize);
                        }
                    }
                }
            }
        }
    }


    #[inline]
    fn is_free(&self, c: Coord) -> bool {
        !self.occupancy.contains_key(&c)
    }

    #[inline]
    fn free(&mut self, c: Coord) {
        self.occupancy.remove(&c);
    }

    #[inline]
    fn occupy(&mut self, c: Coord, tid: ThreadId) {
        self.occupancy.insert(c, tid);
    }
}


            fn any_coord(vec: Vec<Coord>) -> Coord {
        vec.choose(&mut rand::rng()).cloned().unwrap()
    }
