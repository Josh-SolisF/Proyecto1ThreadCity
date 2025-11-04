// tests_utils.rs (o en el mismo módulo de tests)
use mypthreads::mythread::mymutex::MyMutex;
use crate::cityblock::Block;
use crate::cityblock::bridge::BridgeBlock;
use crate::cityblock::bridge::control::Control;
use crate::cityblock::dock::DockBlock;
use crate::cityblock::map::Map;
use crate::cityblock::nuclearplant::NuclearPlantBlock;
use crate::cityblock::road::RoadBlock;
use crate::cityblock::shopblock::ShopBlock;
use crate::cityblock::water::WaterBlock;

// Ajusta si tu Coord se crea distinto
#[inline]
pub fn c(r: usize, k: usize) -> crate::cityblock::coord::Coord {
    crate::cityblock::coord::Coord::new(r as i16, k as i16)
}
 
/// 1x4: Road - Road - Road - Shop

pub fn build_column_map_4x1() -> Map {
    // 4 filas, 1 columna ⇒ grid.len() = 4 (height), grid[0].len() = 1 (width)
    let mut grid: Vec<Vec<Box<dyn Block>>> = Vec::new();
    grid.push(vec![ Box::new(RoadBlock::new(0)) ]); // (x=0, y=0)
    grid.push(vec![ Box::new(RoadBlock::new(1)) ]); // (0,1)
    grid.push(vec![ Box::new(RoadBlock::new(2)) ]); // (0,2)
    grid.push(vec![ Box::new(ShopBlock::new(3, Vec::new())) ]); // (0,3)
    Map::build_custom(grid)
}


/// 1x3: Road - Bridge - Road  (útil para probar NextIsBridge)

pub fn build_column_map_with_bridge() -> Map {
    let mut grid: Vec<Vec<Box<dyn Block>>> = Vec::new();
    grid.push(vec![ Box::new(RoadBlock::new(10)) ]); // (0,0)
    let control = Control::without_traffic(false);   // policy: AnyVehicle (cars allowed)
    grid.push(vec![ Box::new(BridgeBlock::new(11, control, MyMutex::new())) ]); // (0,1)
    grid.push(vec![ Box::new(RoadBlock::new(12)) ]); // (0,2)
    Map::build_custom(grid)
}


#[cfg(test)]
mod tests {
    use mypthreads::mythread::mythread::ThreadId;
    use super::*; // importa TrafficHandler, Car, etc.
    use crate::city::traffic_handler::TrafficHandler;
    use crate::vehicle::car::Car;
    use crate::vehicle::vehicle::{Vehicle, MoveIntent};
    /// Crea un handler, inserta un coche determinísticamente, y fija ocupación/reloj.
    fn setup_handler_with_car(origin: (usize,usize), dest: (usize,usize))
                              -> (TrafficHandler<'static>, ThreadId)
    {
        // Para prestar 'static en tests, guardamos el mapa en un Box y luego
        // hacemos 'leak' controlado SOLO dentro del test (válido en tests).
        let map = Box::new(build_column_map_4x1());
        let map_ref: &'static mut Map = Box::leak(map);

        let mut handler = TrafficHandler::new(map_ref, vec![]);
        let tid: ThreadId = 1; // ajusta si ThreadId no es usize

        // Construimos el auto de forma determinista, sin random
        let origin_c = c(origin.0, origin.1);
        let dest_c   = c(dest.0, dest.1);
        let mut car = Car::new(origin_c, dest_c);
        car.initialize(&**handler.map.borrow_mut(), tid);

        // Insertamos manualmente (evitamos new_car() que usa random)
        handler.vehicles.insert(tid, Box::new(car));

        (handler, tid)
    }

    #[test]
    fn car_moves_one_cell() {
        // Mapa: (0,0) -> (0,1) -> (0,2) -> (0,3 Shop) ; speed=1 celda/s => 1000ms por paso
        let (mut handler, tid) = setup_handler_with_car((0,0), (0,3));

        handler.advance_time();

        let v = handler.vehicles.get(&tid).unwrap();
        let pos = v.current();
        assert_eq!(pos, c(0,1), "El carro debe haber avanzado de (0,0) a (0,1)");

        // Ocupación actualizada
        assert_eq!(handler.vehicles.get(&tid).unwrap().base().path_idx, 2, "La celda nueva debe estar ocupada por el carro");
    }

    #[test]
    fn plan_next_marks_bridge_as_special() {
        // Camino: Road -> Bridge -> Road. El plan_next desde (0,0) debe devolver NextIsBridge((0,1)).
        let map = build_column_map_with_bridge();

        let mut car = Car::new(c(0,0), c(0,2));
        let tid: ThreadId = 99;
        car.initialize(&map, tid);

        match car.base().plan_next(&map) {
            MoveIntent::NextIsBridge { coord } => assert_eq!(coord, c(0,1)),
            other => panic!("Esperaba NextIsBridge((0,1)), pero obtuve {:?}", other),
        }
    }

    #[test]
    fn car_reaches_destination_after_enough_steps() {
        let (mut handler, tid) = setup_handler_with_car((0,0), (0,3));

        handler.advance_time(); // -> (0,1)
        handler.advance_time(); // -> (0,2)
        handler.advance_time(); // -> (0,3) destino

        // Debe estar en successes
        let v = handler.successes.pop().unwrap();
        assert_eq!(v, tid, "Si estaba al frente de exitos, llegó");
    }
}
