use std::any::Any;
use std::collections::{HashMap, HashSet, VecDeque};
use mypthreads::mythread::mythread::ThreadId;
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::vehicle::vehicle_type::VehicleType;

pub struct VehicleBase {
    pub(crate) current_position: Coord,
    pub(crate) vehicle_type: VehicleType,
    pub(crate) destination: Coord,
    pub(crate) speed: u8,
    pub(crate) path: Option<Vec<Coord>>,
    pub(crate) thread_id: Option<ThreadId>,
}
impl VehicleBase {
    pub fn new(origin: Coord, destination: Coord, speed: u8, vehicle_type: VehicleType) -> VehicleBase {
        Self {
            vehicle_type,
            current_position: origin,
            destination,
            speed,
            path: None,
            thread_id: None,
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
                self.path = Some(path);
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
        
        self.path = None
    }

}
pub trait Vehicle: Any {
    fn get_type(&self) -> &VehicleType;
    fn as_any(&self) -> &dyn Any;
    fn initialize(&mut self, map: &Map, tid: ThreadId);
}