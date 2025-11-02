use std::collections::HashMap;
use rand::prelude::IndexedRandom;
use mypthreads::mythread::mythread::ThreadId;
use crate::cityblock::block_type::BlockType::{Dock, Road, Shops, Water};
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::vehicle::car::Car;
use crate::vehicle::vehicle::{MoveIntent, Occupancy, Vehicle};

pub struct TrafficHandler<'a> {
    pub(crate) vehicles: HashMap<ThreadId, Box<dyn Vehicle>>,
    road_coords: Vec<Coord>,
    shops_coords: Vec<Coord>,
    water_spawns: Vec<Coord>,
    dock: Coord,
    pub(crate) map: &'a Map,

    pub(crate) occupancy: HashMap<Coord, ThreadId>,
    pub(crate) time_acc: HashMap<ThreadId, f32>, // ms acumulados por vehículo



}

impl<'a> TrafficHandler<'a> {
    pub fn new(map: &'a Map, water_spawns: Vec<Coord>) -> Self {
        Self {
            vehicles: HashMap::new(),
            road_coords: map.find_blocks(Road),
            shops_coords: map.find_blocks(Shops),
            water_spawns,
            dock: map.find_blocks(Dock).get(0).unwrap().clone(),
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
    pub fn tick(&mut self, dt_ms: u64) {
        let dt = dt_ms as f32;

        // Itera estable (o random si prefieres fairness)
        let tids: Vec<ThreadId> = self.vehicles.keys().cloned().collect();

        for tid in tids {
            let Some(v) = self.vehicles.get_mut(&tid) else { continue; };

            // 1) Acumular tiempo
            let acc = self.time_acc.entry(tid).or_insert(0.0);
            *acc += dt;

            let step_time = Self::step_ms(v.base().speed_cps());
            if *acc < step_time {
                continue; // aun no le toca moverse
            }

            // 2) Intentar un paso (una celda)
            match v.base().plan_next(self.map) {
                MoveIntent::Arrived => {
                    // opcional: remover vehículo, contabilidad, etc.
                }
                MoveIntent::NoPath | MoveIntent::BlockedByPolicy => {
                    // no se mueve; puedes decidir si drenas el acc o lo mantienes
                    *acc = step_time.min(*acc); // queda listo para reintentar pronto
                }
                MoveIntent::AdvanceTo { coord: next } => {
                    // Verificar ocupación
                    if self.is_free(next) {
                        // COMMIT: actualizar occupancy y el vehículo
                        let from = v.base().current();
                        self.free(from);
                        self.occupy(next, tid);
                        v.base_mut().commit_advance(next);
                        *acc -= step_time; // consumimos el paso
                    } else {
                        // ocupado: esperar
                        *acc = step_time.min(*acc);
                    }
                }
                MoveIntent::NextIsBridge { coord: next } => {
                    // Por ahora: tratarlo como carretera normal
                    // (o invocar tu BridgeBlock aquí para pedir permiso)
                    if self.is_free(next) {
                        let from = v.base().current();
                        self.free(from);
                        self.occupy(next, tid);
                        v.base_mut().commit_advance(next);
                        *acc -= step_time;
                    } else {
                        *acc = step_time.min(*acc);
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
