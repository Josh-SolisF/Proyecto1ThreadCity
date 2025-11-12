use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use rand::Rng;
use mypthreads::mythread::mutexlockkind::MyMutexAttr;
use mypthreads::mythread::mymutex::MyMutex;
use mypthreads::mythread::mypthread::MyPThread;
use mypthreads::mythread::mythread::{AnyParam, MyTRoutine, MyThreadAttr, ThreadId};
use crate::city::traffic_handler::TrafficHandler;
use crate::cityblock::block_type::BlockType::NuclearPlant;
use crate::cityblock::bridge::BridgeBlock;
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::cityblock::nuclearplant::NuclearPlantBlock;
use crate::cityblock::nuclearplant::plant_status::PlantStatus::Boom;
use crate::cityblock::nuclearplant::supply_spec::SupplySpec;

pub struct SimulationController {
    pub(crate) traffic: TrafficHandler,
    pub(crate) nuclear_plants: Vec<Coord>,
    pub(crate) with_traffic_bridge: Coord,
    pub(crate) map: Rc<RefCell<Map>>,
    pub(crate) my_pthread: MyPThread,
}

impl SimulationController {
    pub fn new() -> Self {
        let mut mpt = MyPThread::new();
        let mut mut1 = MyMutex::new();
        let mut mut2 = MyMutex::new();
        let mut mut3 = MyMutex::new();
        let mut mut_at1 = MyMutexAttr::new(0);
        let mut mut_at2 = MyMutexAttr::new(0);
        let mut mut_at3 = MyMutexAttr::new(0);

        unsafe {
            mpt.my_mutex_init(&mut mut1, &mut mut_at1);
            mpt.my_mutex_init(&mut mut2, &mut mut_at2);
            mpt.my_mutex_init(&mut mut3, &mut mut_at3);

        }

        let city_map = Rc::new(RefCell::new(Map::map_25x25_with_all_blocks(mut1, mut2, mut3)));
        let plants = city_map.borrow().find_blocks(NuclearPlant);
        let traf = TrafficHandler::new(city_map.clone(),
                                       vec![ Coord::new(23, 0),
                                                         Coord::new(24,9),
                                                         Coord::new(24,21)]);
        Self {
            traffic: traf,
            nuclear_plants: plants,
            map: city_map,
            my_pthread: mpt,
            with_traffic_bridge: Coord::new(1, 10),
        }
    }
    pub fn advance_time(&mut self, frames: u8) {
        for _ in 0..frames {
            {
                let mut map = self.map.borrow_mut();
                if let Some(b) = map.get_block_at(self.with_traffic_bridge).unwrap().as_any().downcast_mut::<BridgeBlock>() {
                   b.advance_time(1);
                }
            }
            let mut scheds : HashMap<Coord, Vec<SupplySpec>> = HashMap::new();
            for coord in self.nuclear_plants.clone().iter() {
                let mut map = self.map.borrow_mut();
                if let Some(p) = map.get_block_at(*coord).unwrap().as_any().downcast_mut::<NuclearPlantBlock>() {
                    let status = p.advance_time(1);
                    if status != Boom && p.requires.len() > 0{
                        if p.scheduled_kinds.is_empty() {
                            scheds.insert(*coord, p.requires.clone());
                        }
                    }
                }
            }
            if !scheds.is_empty() {
                self.generate_trucks(scheds);
            }
            self.traffic.advance_time();
            self.check_traffic();
            self.generate_vehicles();
        }
    }
    fn generate_vehicles(&mut self) {
        if self.traffic.vehicles.len() > 70 {
            return;
        }
        let mut rng = rand::rng();
        if rng.random_bool(0.74) {
            let option = rng.random_range(0..3);
            let tid = self.initialize_a_thread();
            match option {
                0 => {
                    self.traffic.new_car(tid);
                    return;
                }
                1 => {
                    self.traffic.new_ambulance(tid);
                    return;
                }
                _ => {
                    self.traffic.new_ship(tid);
                }
            }
        }
    }
    fn generate_trucks(&mut self, scheds : HashMap<Coord, Vec<SupplySpec>>) {
        for sched in scheds {
            let specs = sched.1;
            for spec in specs {
                let tid = self.initialize_a_thread();
                self.traffic.new_truck(tid, sched.0, spec);
            }
        }
    }
    fn initialize_a_thread(&mut self) -> ThreadId {
        extern "C" fn dummy_vehicle(arg: *mut AnyParam) -> *mut AnyParam {
            println!("Im mooving");
            arg as *mut AnyParam
        }

        let mut tid: ThreadId = 0;
        let mut attr: MyThreadAttr = MyThreadAttr::new(0, 30);
        let routine: MyTRoutine = dummy_vehicle;
        let args: *mut AnyParam = tid as *mut AnyParam;
        unsafe {
            self.my_pthread.my_thread_create(&mut tid,
                                             &mut attr,
                                             routine,
                                             args,
                                             None);
        }
        tid
    }
    fn check_traffic(&mut self) {
        let frame = self.traffic.passed_frames;
        if let Some(fails) = self.traffic.fails.get(&frame) {
            println!("Fails in frame {:?}: {:?} ", frame, fails);
        }
        if let Some(fails) = self.traffic.fails_by_type.keys().next() {
            println!("Fails by type {:?}: {:?} ", fails, self.traffic.fails_by_type.get(fails).unwrap());
        }
        if let Some(successes) = self.traffic.successes.get(&frame) {
            println!("Successes in frame {:?}: {:?} ", frame, successes);
        }
    }
}