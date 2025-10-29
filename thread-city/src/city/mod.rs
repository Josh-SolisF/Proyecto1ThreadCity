use mypthreads::mythread::mypthread;
use mypthreads::mythread::mypthread::MyPThread;
use mypthreads::{RoundRobinScheduler, Scheduler};
use mypthreads::scheduler::{LotteryScheduler, RealTimeScheduler};
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
            Box::new(RoundRobinScheduler),
            Box::new(LotteryScheduler),
            Box::new(RealTimeScheduler),
        ];
        ThreadCity {
            map: Map::build_default(),
            my_pthread: MyPThread::new(schedulers),
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
        todo!("Debe avanzar el tiempo de simulación para cada elemento que requiera del tiempo para funcionar.")
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