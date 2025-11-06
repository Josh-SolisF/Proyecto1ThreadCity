use std::cell::RefCell;
use std::rc::Rc;
use crate::city::traffic_handler::TrafficHandler;
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::cityblock::nuclearplant::NuclearPlantBlock;

pub struct SimulationController<'a> {
    pub(crate) traffic: TrafficHandler<'a>,
    pub(crate) nuclear_plants: Vec<NuclearPlantBlock>,
    pub(crate) map: Rc<RefCell<Map>>,
}

impl<'a> SimulationController<'a> {
    pub fn new() -> Self {
        let mut city_map: Map = Map::map_25x25_with_all_blocks();
        let mut plants = city_map.get_plants();
        Self {
            traffic: TrafficHandler::new(&mut city_map,
                                         vec![Coord::new(23, 0),
                                                    Coord::new(24,9), Coord::new(24,21)]),
            nuclear_plants: plants,
            map: city_map,
        }
    }
    pub fn advance_time(&mut self, frames: u8) {
        for _ in 0..frames {
            for mut p in self.nuclear_plants {
                p.advance_time(1);
            }
            self.traffic.advance_time()
        }
    }
}