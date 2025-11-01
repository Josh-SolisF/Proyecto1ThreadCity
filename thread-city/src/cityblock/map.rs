use std::collections::VecDeque;
use crate::cityblock::block::{Block};


use crate::cityblock::block_type::BlockType;
use crate::cityblock::block_type::BlockType::Road;
use crate::cityblock::coord::Coord;
use crate::cityblock::road::RoadBlock;
use crate::cityblock::transport_policy::TransportPolicy;
use crate::vehicle::vehicle_type::VehicleType;

pub struct Map {
    pub(crate) grid: Vec<Vec<Box<dyn Block>>>,
    pub(crate) height: i16,
    pub(crate) width: i16,
}

impl Map {
    pub fn build_custom(grid: Vec<Vec<Box<dyn Block>>>) -> Map {
        let height = grid.len() as i16;
        let width = grid[0].len() as i16;
        Self {
            grid,
            height,
            width,
        }
    }
    pub fn in_bounds(&self, coord: Coord) -> bool {
        coord.x >= 0 && coord.y >= 0 && coord.x < self.width as i16 && coord.y < self.height as i16
    }
    pub fn policy_at(&self, coord: Coord) -> Option<TransportPolicy> {
        if self.in_bounds(coord) {
            return Some(*self.grid[coord.y as usize][coord.x as usize].get_policy());
        }
        None
    }
    pub fn neighbors(&self, coord: Coord) -> Vec<Coord> {
        let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        deltas.iter()
            .map(|(dx, dy)| Coord::new(coord.x + dx, coord.y + dy))
            .filter(|coord| self.in_bounds(*coord))
            .collect()
    }
    pub fn find_blocks(&self, target: BlockType) -> Vec<Coord> {
        let mut matches = Vec::new();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                if block.get_type() == &target {
                    matches.push(Coord::new(x as i16, y as i16));
                }
            }
        }

        matches
    }
}