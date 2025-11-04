use std::any::Any;
use std::cell::RefCell;
use mypthreads::mythread::mythread::ThreadId;
use crate::cityblock::coord::Coord;
use crate::cityblock::map::Map;
use crate::vehicle::vehicle::{MoveIntent, PatienceLevel, Vehicle, VehicleBase};
use crate::vehicle::vehicle::PatienceLevel::{Critical, Low, Maxed, Starved};
use crate::vehicle::vehicle_type::VehicleType;
use crate::vehicle::vehicle_type::VehicleType::CarE;

pub struct Car {
    pub(crate) base: VehicleBase,
}

impl Car {
    pub fn new(origin: Coord, destination: Coord) -> Self {
        Self {
            base: VehicleBase::new(origin, destination, CarE, 8),
        }
    }
}

impl Vehicle for Car {
    fn get_type(&self) -> &VehicleType {
        &self.base.vehicle_type
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
    fn initialize(&mut self, map: &Map, tid: ThreadId) {
        self.base.calculate_path(map);

        // Útil para detectar por qué algunos tests dicen "NoPath"
        debug_assert!(
            self.base.path.is_some(),
            "calculate_path no generó ruta (origin={:?} dest={:?})",
            self.base.current_position,
            self.base.destination
        );

        self.base.thread_id = Some(tid);
    }
    fn plan_next_move(&self, map: &Map) -> MoveIntent {
        if self.base.current_position == self.base.destination ||
            self.base.path_idx >= (self.base.path.as_ref().unwrap().len() - 1) {
            return MoveIntent::Arrived;
        }
        self.base.plan_next(map)
    }

    fn try_move(&mut self, next_is_open: bool) -> PatienceLevel {
        if next_is_open {
            self.base.patience = self.base.max_patience;
            self.base.current_position = self.base.path.as_mut().unwrap()[self.base.path_idx];
            self.base.path_idx += 1;
            return Maxed {moved: true};
        }
        self.base.patience -= 1;
        self.calc_patience()
    }

    fn base(&self) -> &VehicleBase { &self.base }
    fn base_mut(&mut self) -> &mut VehicleBase { &mut self.base }

    fn calc_patience(&self) -> PatienceLevel {
        match self.base.patience {
            8 | 7 => Maxed { moved: false },
            6 | 5 | 4 | 3 => Low,
            2 | 1 => Critical,
            _ => {Starved}
        }
    }
}