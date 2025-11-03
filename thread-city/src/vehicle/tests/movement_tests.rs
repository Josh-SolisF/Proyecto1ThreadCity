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
    use crate::vehicle::vehicle::{Vehicle, MoveIntent};

    /// Crea un handler, inserta un coche determinísticamente, y fija ocupación/reloj.
    fn setup_handler_with_car(speed: u8, origin: (usize,usize), dest: (usize,usize))
                              -> (TrafficHandler<'static>, ThreadId)
    {
        // Para prestar 'static en tests, guardamos el mapa en un Box y luego
        // hacemos 'leak' controlado SOLO dentro del test (válido en tests).
        let mut map = Box::new(build_column_map_4x1());        
        let map_ref: &'static mut Map = Box::leak(map);

        let mut handler = TrafficHandler::new(map_ref, vec![]);
        let tid: ThreadId = 1; // ajusta si ThreadId no es usize

        // Construimos el auto de forma determinista, sin random
        let origin_c = c(origin.0, origin.1);
        let dest_c   = c(dest.0, dest.1);
        let mut car = crate::vehicle::car::Car::new(origin_c, dest_c, speed);
        car.initialize(handler.map, tid);

        // Insertamos manualmente (evitamos new_car() que usa random)
        handler.occupancy.insert(origin_c, tid);
        handler.time_acc.insert(tid, 0.0);
        handler.vehicles.insert(tid, Box::new(car));

        (handler, tid)
    }

    #[test]
    fn car_moves_one_cell_when_time_reaches_step_time() {
        // Mapa: (0,0) -> (0,1) -> (0,2) -> (0,3 Shop) ; speed=1 celda/s => 1000ms por paso
        let (mut handler, tid) = setup_handler_with_car(1, (0,0), (0,3));

        // Tick de 1000ms: debe mover exactamente 1 celda
        handler.advance_time(1000);

        let v = handler.vehicles.get(&tid).unwrap();
        let pos = v.base().current();
        assert_eq!(pos, c(0,1), "El carro debe haber avanzado de (0,0) a (0,1)");

        // Ocupación actualizada
        assert!(handler.occupancy.get(&c(0,0)).is_none(), "La celda antigua debe estar libre");
        assert_eq!(handler.occupancy.get(&c(0,1)), Some(&tid), "La celda nueva debe estar ocupada por el carro");
    }

    #[test]
    fn car_respects_speed_and_waits_until_enough_time() {
        // speed=2 celdas/s => cada paso cuesta 500ms
        let (mut handler, tid) = setup_handler_with_car(2, (0,0), (0,3));

        // 1) Aún no alcanza: 400ms < 500ms
        handler.advance_time(400);
        let v = handler.vehicles.get(&tid).unwrap();
        assert_eq!(v.base().current(), c(0,0), "No debe moverse aún (400ms < 500ms)");

        // 2) Acumula otros 100ms => 500ms total; ahora sí se mueve 1 celda
        handler.advance_time(100);
        let v = handler.vehicles.get(&tid).unwrap();
        assert_eq!(v.base().current(), c(0,1), "Debe moverse tras completar 500ms");
    }

    #[test]
    fn car_does_not_move_into_occupied_cell_and_moves_later() {
        // Dos carros en línea: A en (0,0), B en (0,1). Destino común en (0,3).
        // Con tick de 1000ms, B debería moverse primero a (0,2) y A queda bloqueado;
        // en el siguiente tick, A ya puede avanzar a (0,1).
        let mut map = Box::new(build_column_map_4x1());        let map_ref: &'static mut Map = Box::leak(map);
        let mut handler = TrafficHandler::new(map_ref, vec![]);

        let tid_a: ThreadId = 10;
        let tid_b: ThreadId = 20;

        // Auto A
        let mut car_a = crate::vehicle::car::Car::new(c(0,0), c(0,3), 1);
        car_a.initialize(handler.map, tid_a);
        handler.vehicles.insert(tid_a, Box::new(car_a));
        handler.occupancy.insert(c(0,0), tid_a);
        handler.time_acc.insert(tid_a, 0.0);

        // Auto B, justo adelante de A
        let mut car_b = crate::vehicle::car::Car::new(c(0,1), c(0,3), 1);
        car_b.initialize(handler.map, tid_b);
        handler.vehicles.insert(tid_b, Box::new(car_b));
        handler.occupancy.insert(c(0,1), tid_b);
        handler.time_acc.insert(tid_b, 0.0);

        // 1er tick: 1000ms -> B intentará ir a (0,2). A intentará ir a (0,1) pero puede quedar bloqueado
        handler.advance_time(1000);

        // Invariante: no hay colisiones
        let occ = &handler.occupancy;
        let unique_cells: std::collections::HashSet<_> = occ.values().collect();
        assert_eq!(unique_cells.len(), 2, "Cada vehículo debe ocupar una celda distinta");

        // 2do tick: A ya debería poder avanzar a (0,1)
        handler.advance_time(1000);

        let pos_a = handler.vehicles.get(&tid_a).unwrap().base().current();
        let pos_b = handler.vehicles.get(&tid_b).unwrap().base().current();
        assert!(pos_b == c(0,2) || pos_b == c(0,3),
                "B debe haber avanzado (normalmente a (0,2) en el primer tick)");
        assert_eq!(pos_a, c(0,1), "A debe avanzar a (0,1) cuando su celda destino se libera");
    }

    #[test]
    fn plan_next_marks_bridge_as_special() {
        // Camino: Road -> Bridge -> Road. El plan_next desde (0,0) debe devolver NextIsBridge((0,1)).
        let mut map = build_column_map_with_bridge();

        let mut car = crate::vehicle::car::Car::new(c(0,0), c(0,2), 1);
        let tid: ThreadId = 99;
        car.initialize(&map, tid);

        match car.base().plan_next(&map) {
            MoveIntent::NextIsBridge { coord } => assert_eq!(coord, c(0,1)),
            other => panic!("Esperaba NextIsBridge((0,1)), pero obtuve {:?}", other),
        }
    }

    #[test]
    fn car_reaches_destination_after_enough_steps() {
        // Con speed=1, desde (0,0) a (0,3): 3 pasos => 3 ticks de 1000ms.
        let (mut handler, tid) = setup_handler_with_car(1, (0,0), (0,3));

        handler.advance_time(1000); // -> (0,1)
        handler.advance_time(1000); // -> (0,2)
        handler.advance_time(1000); // -> (0,3) destino

        let v = handler.vehicles.get(&tid).unwrap();
        assert_eq!(v.base().current(), c(0,3), "Debe estar en destino");
        // Si en tu handler, al llegar, liberas ocupación y retiras el vehículo,
        // este test se puede adaptar a:
        //  - assert!(handler.vehicles.get(&tid).is_none());
        //  - assert!(handler.occupancy.get(&c(0,3)).is_none() /* o Some(&tid) según tu política */);
    }
}
