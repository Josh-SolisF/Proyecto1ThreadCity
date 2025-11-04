use std::any::Any;
use std::collections::{HashMap, HashSet, VecDeque};
use mypthreads::mythread::mythread::ThreadId;
use crate::cityblock::block_type::BlockType::Bridge;
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::vehicle::vehicle_type::VehicleType;




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveIntent {
    Arrived,
    NextIsBridge { coord: Coord }, // hook para handler de puentes
    AdvanceTo { coord: Coord },    // carretera/tienda/etc.
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatienceLevel {
    Maxed { moved: bool },
    Low,
    Critical,
    Starved,
}


pub type Occupancy = HashMap<Coord, ThreadId>;

pub struct VehicleBase {
    pub(crate) current_position: Coord,
    pub(crate) vehicle_type: VehicleType,
    pub(crate) destination: Coord,
    pub(crate) path: Option<Vec<Coord>>,
    pub(crate) thread_id: Option<ThreadId>,
    pub(crate) max_patience: u8,
    pub(crate) path_idx: usize,
    pub(crate) patience: u8,
}
impl VehicleBase {
    pub fn new(origin: Coord, destination: Coord, vehicle_type: VehicleType, max_patience: u8) -> VehicleBase {
        Self {
            vehicle_type,
            current_position: origin,
            destination,
            path: None,
            thread_id: None,
            path_idx: 0,
            patience: max_patience,
            max_patience,
        }
    }
    
    pub fn calculate_path(&mut self, map: &Map) {
        let mut queue = VecDeque::new();
        let mut came_from: HashMap<Coord, Option<Coord>> = HashMap::new();
        let mut visited = HashSet::new();
        
        queue.push_back(self.current_position);
        came_from.insert(self.current_position, None);
        visited.insert(self.current_position);
        
        while let Some(current) = queue.pop_front() {
            if current == self.destination {
                let mut path: Vec<Coord> = Vec::new();
                let mut c = Some(current);
                while let Some(pos) = c {
                    path.push(pos);
                    c = *came_from.get(&pos).unwrap();
                }
                
                path.reverse();

                let len = path.len();
                self.path = Some(path);
                self.path_idx = if len > 1 { 1 } else { 0 };

                return;
            }
            
            for neighbor in map.neighbors(current) {
                if visited.contains(&neighbor) {
                    continue;
                }
                if neighbor == self.destination {
                    visited.insert(neighbor);
                    came_from.insert(neighbor, Some(current));
                    queue.push_back(neighbor);
                    break;
                }
                if let Some(policy) = map.policy_at(neighbor) {
                    if policy.can_pass(self.vehicle_type) {
                        visited.insert(neighbor);
                        came_from.insert(neighbor, Some(current));
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    #[inline]
    pub fn thread(&self) -> ThreadId { self.thread_id.expect("Vehicle without ThreadId") }

    /// Propone el próximo movimiento (no altera estado global).
    pub fn plan_next(&self, map: & Map) -> MoveIntent {
        let Some(path) = self.path.as_ref() else {
            panic!("Vehicle not initialized!");
        };

        let next = path[self.path_idx];
        if let Some(bt) = map.block_type_at(next) {
            if bt == Bridge {
                return MoveIntent::NextIsBridge { coord: next };
            }
        }

        MoveIntent::AdvanceTo { coord: next }
    }

}

// Trait Vehicle (agrego tick para delegar a advance_time)
pub trait Vehicle: Any {
    fn get_type(&self) -> &VehicleType;
    fn as_any(&mut self) -> &mut dyn Any;
    fn initialize(&mut self, map: &Map, tid: ThreadId);
    fn plan_next_move(&self, map: &Map) -> MoveIntent;
    fn try_move(&mut self, next_is_open: bool) -> PatienceLevel;
    fn base(&self) -> &VehicleBase;
    fn base_mut(&mut self) -> &mut VehicleBase;
    fn current(&self) -> Coord {
        self.base().current_position
    }
    fn calc_patience(&self) -> PatienceLevel;

}


