use mypthreads::mythread::mythread::MyThread;
use crate::cityblock::bridge::traffic_light::TrafficLight;
use crate::vehicle::vehicle_type::VehicleType;

pub struct Control {
    pub(crate) in_traffic_light: Option<TrafficLight>,
    pub(crate) out_traffic_light: Option<TrafficLight>,
    pub has_yield: bool,
}

impl Control {
    pub fn with_traffic(
        interval_in: usize,
        interval_out: usize,
    ) -> Self {
        Self {
            in_traffic_light: Some(TrafficLight::new(interval_in)),
            out_traffic_light: Some(TrafficLight::new(interval_out)),
            has_yield: false,
        }
    }

    pub fn without_traffic(
        has_yield: bool,
    ) -> Self {
        Self {
            in_traffic_light: None,
            out_traffic_light: None,
            has_yield,
        }
    }

    // Hooks para siguiente paso (colas/prioridades reales)
    pub fn allow_in(&mut self, _thread: &MyThread, _vehicle: VehicleType) {
        // TODO: colas por prioridad, fairness, anti-starvation
    }
    pub fn allow_out(&mut self, _thread: &MyThread, _vehicle: VehicleType) {
        // TODO
    }
}