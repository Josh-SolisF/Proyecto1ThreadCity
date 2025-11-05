use std::collections::VecDeque;
use crate::cityblock::block::{Block};


use crate::cityblock::block_type::BlockType;
use crate::cityblock::block_type::BlockType::Road;
use crate::cityblock::coord::Coord;
use crate::cityblock::nuclearplant::NuclearPlantBlock;
use crate::cityblock::nuclearplant::plant_status::PlantStatus;
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

    #[inline]
    pub fn get_block_at(&mut self, c: Coord) -> Option<&mut dyn Block> {
        if self.in_bounds(c) {
            Some(&mut *self.grid[c.y as usize][c.x as usize])
        } else {
            None
        }
    }
    #[inline]
    pub fn block_at(&self, c: Coord) -> Option<&dyn Block> {
        if self.in_bounds(c) {
            Some(&*self.grid[c.y as usize][c.x as usize])
        } else {
            None
        }
    }

    pub fn block_type_at(&self, c: Coord) -> Option<BlockType> {
        self.block_at(c).map(|b| *b.get_type())
        // Si BlockType no fuera Copy, cambia a: .map(|b| b.get_type().clone())
    }

/*
    pub fn block_type_at(&self, c: Coord) -> Option<BlockType> {
        self.cell_at(c).map(|cell| cell.block_type)
    }

*/
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
        let deltas: [(i16, i16); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
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



    /// Ancho en celdas
    #[inline]
    pub fn width_cells(&self) -> i16 { self.width }

    /// Alto en celdas
    #[inline]
    pub fn height_cells(&self) -> i16 { self.height }

    /// (width, height)
    #[inline]
    pub fn size_cells(&self) -> (i16, i16) { (self.width, self.height) }

    /// Itera todas las coordenadas del mapa (de izquierda a derecha, arriba a abajo).
    pub fn coords_iter(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.height).flat_map(move |y| {
            (0..self.width).map(move |x| Coord::new(x, y))
        })
    }

    /// Helper de sólo lectura para GUI:
    /// Si en (coord) hay una NuclearPlant, devuelve su `PlantStatus` (via downcast).
    /// IMPORTANTE: Requiere &mut self porque `Block::as_any` es &mut dyn Any.
    pub fn try_plant_status_at(&mut self, coord: Coord) -> Option<PlantStatus> {
        if !self.in_bounds(coord) { return None; }
        // Obtenemos el bloque de forma mutable para poder hacer as_any().downcast_mut().
        let blk = self.get_block_at(coord)?;
        if blk.get_type() != &BlockType::NuclearPlant {
            return None;
        }
        // downcast al bloque concreto
        if let Some(plant) = blk.as_any().downcast_mut::<NuclearPlantBlock>() {
            Some(plant.plant_status)
        } else {
            None
        }
    }

}