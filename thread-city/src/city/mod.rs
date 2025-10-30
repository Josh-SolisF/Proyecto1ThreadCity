use mypthreads::mythread::mypthread;
use mypthreads::mythread::mypthread::MyPThread;
use mypthreads::{LotteryScheduler, RoundRobinScheduler, Scheduler};
use mypthreads::scheduler::{RealTimeScheduler};
use mypthreads::scheduler::round_robin::RRScheduler;
use crate::city::map::Map;

pub mod supply_kind;
pub mod map;

pub struct ThreadCity {
    pub map: Map,
    pub my_pthread: MyPThread,
}

impl ThreadCity {
    pub fn new() -> ThreadCity {
        let schedulers: Vec<Box<dyn Scheduler>> = vec![
            Box::new(RRScheduler::new()),
            Box::new(LotteryScheduler::new()),
            Box::new(RealTimeScheduler::new()),
        ];
        let map = Map::build_default();

        ThreadCity {
            map: Map::build_default(),
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

    pub fn update_status(&mut self) {
        // avanzar tiempo global
        for row in &mut self.map.grid {
            for block in row {
                block.update(100); // avanzar 100 ms por ciclo
            }
        }
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