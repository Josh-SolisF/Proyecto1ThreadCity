use mypthreads::mythread::mythread::MyThread;
use crate::cityblock::traffic_light::TrafficLight;
use crate::vehicle::vehicle_type::VehicleType;

pub struct Control {
    pub(crate) in_traffic_light: Option<TrafficLight>,
    pub(crate) out_traffic_light: Option<TrafficLight>,
}

impl Control {
    pub fn with_traffic(interval_in: usize, interval_out: usize) -> Self {
        Self {
            in_traffic_light: Some(TrafficLight::new(interval_in)),
            out_traffic_light: Some(TrafficLight::new(interval_out)),
        }
    }
    pub fn without_traffic() -> Self {
        Self {
            in_traffic_light: None,
            out_traffic_light: None,
        }
    }
    
    pub fn allow_in(&mut self, thread: &MyThread, vehicle: VehicleType) {
        todo!()
    }
    pub fn allow_out(&mut self, thread: &MyThread, vehicle: VehicleType) {
        todo!()
    }

}