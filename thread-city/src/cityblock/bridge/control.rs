use mypthreads::mythread::mythread::MyThread;
use crate::cityblock::traffic_light::TrafficLight;
use crate::vehicle::vehicle_type::VehicleType;

pub struct Control {
    pub(crate) in_traffic_light: Option<TrafficLight>,
    pub(crate) out_traffic_light: Option<TrafficLight>,
    pub has_semaphore: bool,
    pub has_yield: bool,
    pub is_open: bool,
}

impl Control {
    pub fn with_traffic(interval_in: usize, interval_out: usize) -> Self {
        Self {
            in_traffic_light: Some(TrafficLight::new(interval_in)),
            out_traffic_light: Some(TrafficLight::new(interval_out)),
            has_semaphore, has_yield, is_open
        }
    }
    pub fn without_traffic() -> Self {
        Self {
            in_traffic_light: None,
            out_traffic_light: None,
            has_semaphore, has_yield, is_open
        }
    }
    
    pub fn allow_in(&mut self, thread: &MyThread, vehicle: VehicleType) {
        todo!()
    }
    pub fn allow_out(&mut self, thread: &MyThread, vehicle: VehicleType) {
        todo!()
    }

    pub fn can_enter(&self) -> bool {
        if !self.is_open {
            return false;
        }
        if self.has_semaphore {
            // puedes conectar esto a TrafficLight más adelante
            return true;
        }
        true
    }

    pub fn set_open(&mut self, state: bool) {
        self.is_open = state;
    }
}