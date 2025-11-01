use std::collections::HashMap;
use rand::prelude::IndexedRandom;
use mypthreads::mythread::mythread::ThreadId;
use crate::cityblock::block_type::BlockType::{Dock, Road, Shops, Water};
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::vehicle::car::Car;
use crate::vehicle::vehicle::Vehicle;

pub struct TrafficHandler<'a> {
    vehicles: HashMap<ThreadId, Box<dyn Vehicle>>,
    road_coords: Vec<Coord>,
    shops_coords: Vec<Coord>,
    water_spawns: Vec<Coord>,
    dock: Coord,
    map: &'a Map,
}

impl<'a> TrafficHandler<'a> {
    pub fn new(map: &'a Map, water_spawns: Vec<Coord>) -> Self {
        Self {
            vehicles: HashMap::new(),
            road_coords: map.find_blocks(Road),
            shops_coords: map.find_blocks(Shops),
            water_spawns,
            dock: map.find_blocks(Dock).get(0).unwrap().clone(),
            map
        }
    }
    pub fn new_car(&mut self, tid: ThreadId) {
        let origin = Self::any_coord(self.road_coords.clone());
        let destination = Self::any_coord(self.shops_coords.clone());
        let mut nc = Car::new(origin, destination, 1);
        nc.initialize(self.map, tid);
        self.vehicles.insert(tid, Box::new(nc));
    }

    fn any_coord(vec: Vec<Coord>) -> Coord {
        vec.choose(&mut rand::rng()).cloned().unwrap()
    }
}