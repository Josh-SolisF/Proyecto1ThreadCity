use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;
use crate::cityblock::block::BlockBase;


use crate::cityblock::block_type::BlockType;
use crate::cityblock::coord::Coord;
use crate::cityblock::transport_policy::TransportPolicy;
use crate::vehicle::vehicle_type::VehicleType;

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
                //  Rio central
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
                    occupied: AtomicBool::from(false),
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
                block.occupied = AtomicBool::from(true);
                return true;
            }
        }
        false
    }

    pub fn neighbors(&self, c: &Coord) -> Vec<Coord> {
        let mut v = Vec::with_capacity(4);
        let x = c.x as isize;
        let y = c.y as isize;
        let dirs = [(1,0),(-1,0),(0,1),(0,-1)];
        for (dx,dy) in dirs {
            let nx = x + dx;
            let ny = y + dy;
            if nx>=0 && ny>=0 && nx < self.width as isize && ny < self.height as isize {
                v.push(Coord::new(nx as u16, ny as u16));
            }
        }
        v
    }


    pub fn is_traversable_for(&self, coord: &Coord, vt: VehicleType) -> bool {
        if let Some(b) = self.get_block(coord) {
            match (b.block_type, vt) {
                (BlockType::Water, VehicleType::Ship) => true,
                (BlockType::Dock,  VehicleType::Ship) => true,
                (BlockType::Bridge, _)                => true, // controller manda
                (BlockType::Road,   VehicleType::Car | VehicleType::Ambulance | VehicleType::Truck) => true,
                (BlockType::ShopBlock, VehicleType::Car | VehicleType::Ambulance) => true,
                (BlockType::NuclearPlant, VehicleType::Truck) => true,
                _ => false,
            }
        } else { false }
    }


    pub fn release_block(&mut self, coord: &Coord) {
        if let Some(block) = self.get_block_mut(coord) {
            block.occupied = AtomicBool::from(false);
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




pub fn shortest_path(&self, origin: Coord, dest: Coord, vt: VehicleType) -> Option<Vec<Coord>> {
    let mut prev = vec![vec![None::<Coord>; self.width]; self.height];
    let mut q = VecDeque::new();
    q.push_back(origin);
    prev[origin.y as usize][origin.x as usize] = Some(origin);

    while let Some(u) = q.pop_front() {
        if u == dest { break; }
        for v in self.neighbors(&u) {
            if !self.is_traversable_for(&v, vt) { continue; }
            if prev[v.y as usize][v.x as usize].is_none() {
                prev[v.y as usize][v.x as usize] = Some(u);
                q.push_back(v);
            }
        }
    }

    if prev[dest.y as usize][dest.x as usize].is_none() { return None; }
    // reconstrucción
    let mut path = vec![dest];
    let mut cur = dest;
    while cur != origin {
        cur = prev[cur.y as usize][cur.x as usize].unwrap();
        path.push(cur);
    }
    path.reverse();
    Some(path)
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