use std::cell::RefCell;
use std::rc::Rc;
use crate::city::traffic_handler::TrafficHandler;
use crate::cityblock::block_type::BlockType::NuclearPlant;
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::cityblock::nuclearplant::NuclearPlantBlock;

pub struct SimulationController {
    pub(crate) traffic: TrafficHandler,
    pub(crate) nuclear_plants: Vec<Coord>,
    pub(crate) map: Rc<RefCell<Map>>,
}

impl SimulationController {
    pub fn new() -> Self {
        let city_map = Rc::new(RefCell::new(Map::map_25x25_with_all_blocks()));
        let plants = city_map.borrow().find_blocks(NuclearPlant);
        let traf = TrafficHandler::new(city_map.clone(),
                                       vec![  Coord::new(23, 0),
                                              Coord::new(24,9),
                                              Coord::new(24,21)]);
        Self {
            traffic: traf,
            nuclear_plants: plants,
            map: city_map,
        }
    }
    pub fn advance_time(&mut self, frames: u8) {
        let mut map = self.map.borrow_mut();
        for _ in 0..frames {
            for coord in self.nuclear_plants.clone().iter() {
                if let Some(mut p) = map.get_block_at(*coord).unwrap().as_any().downcast_mut::<NuclearPlantBlock>() {
                    p.advance_time(1);
                }
            }
            self.traffic.advance_time()
        }
    }
}