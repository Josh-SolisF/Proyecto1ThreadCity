use std::collections::VecDeque;
use crate::cityblock::block::{Block};


use crate::cityblock::block_type::BlockType;
use crate::cityblock::coord::Coord;
use crate::cityblock::transport_policy::TransportPolicy;
use crate::vehicle::vehicle_type::VehicleType;

pub struct Map {
    pub(crate) grid: Vec<Vec<Box<dyn Block>>>,
    pub(crate) height: usize,
    pub(crate) width: usize,
}

impl Map {
    pub fn build_default() -> Map {
        todo!("Generar la ciudad a mano zzzz...")
    }
    pub fn build_custom(grid: Vec<Vec<Box<dyn Block>>>) -> Map {
        let height = grid.len();
        let width = grid[0].len();
        Self {
            grid,
            height,
            width,
        }
    }
}