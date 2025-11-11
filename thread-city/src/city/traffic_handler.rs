use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use rand::prelude::IndexedRandom;
use mypthreads::mythread::mythread::ThreadId;
use crate::cityblock::block_type::BlockType::{Bridge, Dock, Road, Shops, Water, NuclearPlant};
use crate::cityblock::bridge::bridge_permision_enum::EntryOutcome;
use crate::cityblock::bridge::bridge_permision_enum::EntryOutcome::{GrantedFor, Occupied};
use crate::cityblock::bridge::BridgeBlock;
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::cityblock::nuclearplant::NuclearPlantBlock;
use crate::cityblock::road::RoadBlock;
use crate::cityblock::water::WaterBlock;
use crate::vehicle::car::Car;
use crate::vehicle::cargotruck::CargoTruck;
use crate::vehicle::vehicle::{MoveIntent, PatienceLevel, Vehicle};
use crate::vehicle::vehicle::PatienceLevel::{Maxed, Low, Critical, Starved};
use crate::vehicle::vehicle_type::VehicleType;
use crate::vehicle::vehicle_type::VehicleType::{ShipE, TruckE};

pub struct TrafficHandler{
    pub(crate) vehicles: HashMap<ThreadId, Box<dyn Vehicle>>,
    road_coords: Vec<Coord>,
    shops_coords: Vec<Coord>,
    water_spawns: Vec<Coord>,
    dock: Option<Coord>,
    pub(crate) map: Rc<RefCell<Map>>,
    pub(crate) passed_frames: usize,
    pub(crate) fails_by_type: HashMap<VehicleType, u8>,
    pub(crate) fails: Vec<ThreadId>,
    pub(crate) successes: Vec<ThreadId>,
}

impl TrafficHandler {
    pub fn new(map: Rc<RefCell<Map>>
               , water_spawns: Vec<Coord>) -> Self {
        let dock = map.borrow().find_blocks(Dock).get(0).cloned();
        let roads = map.borrow().find_blocks(Road).clone();
        let shops = map.borrow().find_blocks(Shops).clone();
        
        Self {
            vehicles: HashMap::new(),
            road_coords: roads,
            shops_coords: shops,
            water_spawns,
            dock,
            map,
            passed_frames: 0,
            fails_by_type: HashMap::new(),
            successes: Vec::new(),
            fails: Vec::new(),
        }
    }

    pub fn new_car(&mut self, tid: ThreadId) {
        let mut origin = Self::any_coord(self.road_coords.clone());
        let mut map = self.map.borrow_mut();
        let mut rb = map.get_block_at(origin);
        let mut road: &mut RoadBlock = rb.unwrap().as_any().downcast_mut::<RoadBlock>().unwrap();
        loop {
            if road.is_available() {
                road.consume_space();
                break;
            }
            origin = Self::any_coord(self.road_coords.clone());
            rb = map.get_block_at(origin);
            road = rb.unwrap().as_any().downcast_mut::<RoadBlock>().unwrap();
        }

        let destination = Self::any_coord(self.shops_coords.clone());
        let mut nc = Car::new(origin, destination);
        nc.initialize(&self.map.borrow_mut(), tid);

        self.vehicles.insert(tid, Box::new(nc));
    }

    fn any_coord(vec: Vec<Coord>) -> Coord {
        vec.choose(&mut rand::rng()).cloned().unwrap()
    }

    pub fn advance_time(&mut self) {
        //  Avanza en la carretera u prepara los puentes
        let mut planned_for_bridge: HashMap<Coord, Vec<ThreadId>> = HashMap::new();
        for tid in self.vehicles.keys().cloned().collect::<Vec<_>>() {
            if let Some(v_type) = self.vehicles.get(&tid) {
                match v_type.get_type() {
                    ShipE => {
                        if let Some(expected) = self.aquatic_intention(&tid) {
                            planned_for_bridge.entry(expected.0).or_insert(Vec::new()).push(expected.1)
                        }
                    },
                    _ => {
                        if let Some(expected) = self.road_intention(&tid) {
                            planned_for_bridge.entry(expected.0).or_insert(Vec::new()).push(expected.1)
                        }
                    }
                }
            } else {
                continue;
            }
        }

        // Tratar de entrar en los puentes
        for key in planned_for_bridge.keys() {
            let t_keys = planned_for_bridge.get_key_value(key).unwrap().1;
            let candidates: Vec<_> = t_keys
                .iter()
                .filter_map(|k| self.vehicles.get(k))
                .collect();
            let outcome = self.map.borrow_mut().get_block_at(*key).unwrap().as_any().downcast_mut::<BridgeBlock>().unwrap().request_entry(candidates);

            match outcome {
                GrantedFor { tid } => {
                    self.vehicles.get_mut(&tid).unwrap().try_move(true);
                }
                Occupied => {}
            }
        }

    }


    fn road_intention(&mut self, tid: &ThreadId) -> Option<(Coord, ThreadId)> {
        let mut map = self.map.borrow_mut();
        let (intent, from, v_type) = {
            let vref: &Box<dyn Vehicle> = self.vehicles.get_mut(tid)?;
            (vref.plan_next_move(&map), vref.current(), *vref.get_type())
        };
        match intent {
            MoveIntent::Arrived => {
                if self.vehicles.get(tid).unwrap().get_type() == &TruckE {
                    let truck = self.vehicles.get_mut(tid).unwrap().as_any().downcast_mut::<CargoTruck>().unwrap();
                    self.map.borrow_mut().get_block_at(truck.base().destination).unwrap()
                        .as_any().downcast_mut::<NuclearPlantBlock>()
                        .unwrap().commit_delivery(truck)
                }
                self.successes.push(*tid);
                self.vehicles.remove(&tid);
                None
            }
            MoveIntent::AdvanceTo { coord: to } => {
                let v_patience= {
                    let v_mut = self.vehicles.get_mut(&tid);
                    v_mut.as_ref()?.calc_patience()
                };

                if let Some(bridge) = map.get_block_at(from).unwrap().as_any().downcast_mut::<BridgeBlock>() {
                    if self.handle_bridge_exit(to, v_type, v_patience, bridge) {
                        let get_out: &mut RoadBlock = map.get_block_at(to).unwrap().as_any().downcast_mut::<RoadBlock>().unwrap();
                        self.vehicles.get_mut(&tid)?.try_move(get_out.consume_space());
                    }
                    return None;
                }

                let next_rbl: &mut RoadBlock = map.get_block_at(to).unwrap().as_any().downcast_mut::<RoadBlock>().unwrap();
                let result = self.vehicles.get_mut(&tid)?.try_move(next_rbl.consume_space());
                match result {
                     Maxed { moved } => {
                         if moved {
                             let current_rbl = map.get_block_at(from).unwrap().as_any().downcast_mut::<RoadBlock>().unwrap();
                            current_rbl.liberate_space();
                         }
                     }
                    Starved => {
                        *self.fails_by_type.entry(v_type).or_insert(0) += 1;
                        self.fails.push(*tid);
                        self.vehicles.remove(&tid);
                    }
                    _ => {}
                }
                None
            }
            MoveIntent::NextIsBridge { coord: to } => {
                Some((to, *tid))
            }
        }
    }
    fn aquatic_intention(&mut self, tid: &ThreadId) -> Option<(Coord, ThreadId)> {
        let mut map = self.map.borrow_mut();
        let (intent, from, v_type) = {
            let vref: &Box<dyn Vehicle> = self.vehicles.get_mut(tid)?;
            (vref.plan_next_move(&map), vref.current(), *vref.get_type())
        };
        match intent {
            MoveIntent::Arrived => {
                self.vehicles.remove(&tid);
                None
            }
            MoveIntent::AdvanceTo { coord: to } => {
                let v_patience= {
                    let v_mut = self.vehicles.get_mut(&tid);
                    v_mut.as_ref()?.calc_patience()
                };

                if let Some(bridge) = map.get_block_at(from).unwrap().as_any().downcast_mut::<BridgeBlock>() {
                    if self.handle_bridge_exit(to, v_type, v_patience, bridge) {
                        let get_out: &mut WaterBlock = map.get_block_at(to).unwrap().as_any().downcast_mut::<WaterBlock>().unwrap();
                        self.vehicles.get_mut(&tid)?.try_move(get_out.consume_space());
                    }
                    return None;
                }

                let next_rbl: &mut WaterBlock = map.get_block_at(to).unwrap().as_any().downcast_mut::<WaterBlock>().unwrap();
                let result = self.vehicles.get_mut(&tid)?.try_move(next_rbl.consume_space());
                match result {
                    Maxed { moved } => {
                        if moved {
                            let current_rbl = map.get_block_at(from).unwrap().as_any().downcast_mut::<WaterBlock>().unwrap();
                            current_rbl.liberate_space();
                        }
                    }
                    Starved => {
                        *self.fails_by_type.entry(v_type).or_insert(0) += 1;
                        self.fails.push(*tid);
                        self.vehicles.remove(&tid);
                    }
                    _ => {}
                }
                None
            }
            MoveIntent::NextIsBridge { coord: to } => {
                Some((to, *tid))
            }
        }
    }
     fn handle_bridge_exit(&self, to_coord: Coord, v_type: VehicleType, v_pat: PatienceLevel, bridge: &mut BridgeBlock) -> bool {
         let mut binding = self.map.borrow_mut();
         let to: &mut RoadBlock = binding.get_block_at(to_coord).unwrap().as_any().downcast_mut::<RoadBlock>().unwrap();
         to.is_available() && bridge.exit_bridge(v_type, v_pat)
    }
}
