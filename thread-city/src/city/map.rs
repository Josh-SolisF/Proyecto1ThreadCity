use crate::cityblock::block::BlockBase;


use crate::cityblock::block_type::BlockType;
use crate::cityblock::coord::Coord;
use crate::cityblock::transport_policy::TransportPolicy;

pub struct Map {
    pub grid: Vec<Vec<BlockBase>>,
    pub(crate) height: usize,
    pub(crate) width: usize,
}

impl Map {
    pub fn build_default() -> Self {
        let width = 7;
        let height = 7;
        let mut grid: Vec<Vec<BlockBase>> = Vec::new();

        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                let coord = Coord::new(x as u16, y as u16);

                // ===============================
                //  DEFINICIÓN DE BLOQUE FIJO
                // ===============================
                let block_type = if y == height / 2 {
                    // Río central
                    if [1, 3, 5].contains(&x) {
                        BlockType::Bridge
                    } else {
                        BlockType::Water
                    }
                } else if (x == 0 && y == 0) || (x == width - 1 && y == height - 1) {
                    BlockType::NuclearPlant
                } else if (x + y) % 5 == 0 && y < height / 2 {
                    BlockType::ShopBlock
                } else if y > height / 2 && x < 2 {
                    BlockType::Dock
                } else {
                    BlockType::Road
                };

                let policy = match block_type {
                    BlockType::Water => TransportPolicy::Ship,
                    BlockType::Dock => TransportPolicy::Ship,
                    BlockType::Bridge => TransportPolicy::Any,
                    BlockType::Road => TransportPolicy::Any,
                    BlockType::ShopBlock => TransportPolicy::Car,
                    BlockType::NuclearPlant => TransportPolicy::Truck,
                };

                row.push(BlockBase {
                    id: (y * width + x),
                    position: coord,
                    policy,
                    occupied: false,
                    block_type,
                });
            }
            grid.push(row);
        }

        let map = Map { grid, width, height };
        map.print_layout();
        map
    }
    pub fn get_block(&self, coord: &Coord) -> Option<&BlockBase> {
        self.grid.get(coord.y as usize)?.get(coord.x as usize)
    }

    pub fn get_block_mut(&mut self, coord: &Coord) -> Option<&mut BlockBase> {
        self.grid.get_mut(coord.y as usize)?.get_mut(coord.x as usize)
    }

    pub fn occupy_block(&mut self, coord: &Coord) -> bool {
        if let Some(block) = self.get_block_mut(coord) {
            if !block.occupied {
                block.occupied = true;
                return true;
            }
        }
        false
    }

    pub fn release_block(&mut self, coord: &Coord) {
        if let Some(block) = self.get_block_mut(coord) {
            block.occupied = false;
        }
    }
    pub fn print_layout(&self) {
        println!("\n=== THREAD CITY MAP ===\n");
        for row in &self.grid {
            for block in row {
                let symbol = match block.block_type {
                    BlockType::Road => ".",
                    BlockType::Water => "~",
                    BlockType::Bridge => "B",
                    BlockType::ShopBlock => "S",
                    BlockType::Dock => "D",
                    BlockType::NuclearPlant => "N",
                };
                print!("{} ", symbol);
            }
            println!();
        }
        println!();
    }

}

/*
pub struct Map {
    pub(crate) grid: Vec<Vec<BlockBase>>,
}

impl Map {
    pub fn build_default() -> Map {
        todo!("Generar la ciudad a mano zzzz...")
    }
}*/