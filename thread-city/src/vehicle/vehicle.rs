use std::any::Any;
use std::collections::{HashMap, HashSet, VecDeque};
use mypthreads::mythread::mythread::ThreadId;
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::vehicle::vehicle_type::VehicleType;




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveIntent {
    Arrived,
    NoPath,
    BlockedByPolicy,               // el mapa no permite ese tipo de vehículo
    NextIsBridge { coord: Coord }, // hook para handler de puentes
    AdvanceTo { coord: Coord },    // carretera/tienda/etc.
}



pub type Occupancy = HashMap<Coord, ThreadId>;

pub struct VehicleBase {
    pub(crate) current_position: Coord,
    pub(crate) vehicle_type: VehicleType,
    pub(crate) destination: Coord,
    pub(crate) speed: u8,
    pub(crate) path: Option<Vec<Coord>>,
    pub(crate) thread_id: Option<ThreadId>,

    /// Índice de la siguiente celda destino en `path`
    path_idx: usize,

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
            path_idx: 0,

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


        if let Some(ref p) = self.path {
            // Colocar el índice en la PRIMERA celda distinta del origen
            self.path_idx = if p.len() > 1 { 1 } else { 0 };
        }

    }



    #[inline]
    pub fn current(&self) -> Coord { self.current_position }

    #[inline]
    pub fn thread(&self) -> ThreadId { self.thread_id.expect("Vehicle without ThreadId") }


    #[inline]
    pub fn speed_cps(&self) -> f32 { self.speed.max(1) as f32 } // celdas/seg

    /// Ver la siguiente celda objetivo
    #[inline]
    fn next_cell(&self) -> Option<Coord> {
        self.path.as_ref().and_then(|p| p.get(self.path_idx)).cloned()
    }


    /// Propone el próximo movimiento (no altera estado global).
    pub fn plan_next(&self, map: &Map) -> MoveIntent {
        if self.current_position == self.destination {
            return MoveIntent::Arrived;
        }
        let Some(path) = self.path.as_ref() else { return MoveIntent::NoPath; };
        if self.path_idx >= path.len() {
            return MoveIntent::NoPath;
        }
        let next = path[self.path_idx];

        // Política del bloque destino
        if let Some(pol) = map.policy_at(next) {
            if !pol.can_pass(self.vehicle_type) {
                return MoveIntent::BlockedByPolicy;
            }
        } else {
            return MoveIntent::BlockedByPolicy;
        }

        // ¿Es puente?

        if let Some(bt) = map.block_type_at(next) {
            // Ajusta a tus variantes reales de Bridge
            use crate::cityblock::block_type::BlockType::*;
            if matches!(bt, Bridge | Bridge1 | Bridge2 | Bridge3) {
                return MoveIntent::NextIsBridge { coord: next };
            }
        }

        MoveIntent::AdvanceTo { coord: next }
    }

    /// Commit del avance (ahora sí cambia su estado interno).
    /// Lo llama **solo** el TrafficHandler cuando ya se aseguró ocupación/permiso.
    pub fn commit_advance(&mut self, new_pos: Coord) {
        self.current_position = new_pos;
        if let Some(ref p) = self.path {
            self.path_idx = (self.path_idx + 1).min(p.len());
        }
    }



}

// Trait Vehicle (agrego tick para delegar a advance_time)
pub trait Vehicle: Any {
    fn get_type(&self) -> &VehicleType;
    fn as_any(&self) -> &dyn Any;
    fn initialize(&mut self, map: &Map, tid: ThreadId);

    fn base(&self) -> &VehicleBase;
    fn base_mut(&mut self) -> &mut VehicleBase;

}


