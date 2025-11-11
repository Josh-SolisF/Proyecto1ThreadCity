use mypthreads::mythread::mypthread::MyPThread;
use crate::cityblock::map::Map;

pub mod supply_kind;
pub mod traffic_handler;
mod simulation_controller;

pub struct ThreadCity {
    pub map: Map,
    pub my_pthread: MyPThread,
}

impl ThreadCity {
    pub fn new() -> ThreadCity {
        ThreadCity {
            map: Map::build_custom(vec![]),
            my_pthread: MyPThread::new(),
        }
    }


    pub fn start_simulation(&mut self) {
        todo!("Simular")
    }

    pub fn generate_cars(&mut self) {
        todo!()
    }

    pub fn generate_ambulances(&mut self) {
        todo!()
    }

    pub fn generate_ships(&mut self) {
        todo!()
    }

    pub fn pause_simulation(&mut self) {
        todo!("Debe detener la simulación temporalmente")
    }

    pub fn resume_simulation(&mut self) {
        todo!("Debe reanudar la simulación desde donde estaba")
    }

    pub fn stop_simulation(&mut self) -> Result<(), String> {
        todo!("Debe detener a simulación y retornar estadisticas")
    }
        }