use mypthreads::mythread::mythread::MyThread;
use crate::cityblock::traffic_light::TrafficLight;
use crate::vehicle::vehicle_type::VehicleType;
use crate::cityblock::transport_policy::TransportPolicy;

pub struct Control {
    pub(crate) in_traffic_light: Option<TrafficLight>,
    pub(crate) out_traffic_light: Option<TrafficLight>,
    pub has_semaphore: bool,
    pub has_yield: bool,
    pub is_open: bool,
    pub in_policy: Option<TransportPolicy>,
    pub out_policy: Option<TransportPolicy>,
}

impl Control {
    pub fn with_traffic(
        interval_in: usize,
        interval_out: usize,
        has_yield: bool,
        is_open: bool,
        in_policy: Option<TransportPolicy>,
        out_policy: Option<TransportPolicy>,
    ) -> Self {
        Self {
            in_traffic_light: Some(TrafficLight::new(interval_in)),
            out_traffic_light: Some(TrafficLight::new(interval_out)),
            has_semaphore: true,
            has_yield,
            is_open,
            in_policy,
            out_policy,
        }
    }

    pub fn without_traffic(
        has_yield: bool,
        is_open: bool,
        in_policy: Option<TransportPolicy>,
        out_policy: Option<TransportPolicy>,
    ) -> Self {
        Self {
            in_traffic_light: None,
            out_traffic_light: None,
            has_semaphore: false,
            has_yield,
            is_open,
            in_policy,
            out_policy,
        }
    }

    /// Avanza el semáforo (llámalo desde el scheduler/tick)
    pub fn advance_time(&mut self, delta_ms: usize) {
        if let Some(ref mut tl) = self.in_traffic_light {
            tl.advance_time(delta_ms);
        }
        if let Some(ref mut tl) = self.out_traffic_light {
            tl.advance_time(delta_ms);
        }
    }

    /// Prioridades:
    /// - is_open debe ser true.
    /// - in_policy (si existe) debe permitir el tipo.
    /// - Si hay semáforo y está en rojo, sólo Emergency puede pasar.

    pub fn can_enter(&self, _thread: &MyThread, vehicle: VehicleType) -> bool {
        if !self.is_open {
            return false;
        }
        if let Some(pol) = self.in_policy {
            if !pol.allows(vehicle) {
                return false;
            }
        }
        if self.has_semaphore {
            if let Some(ref tl) = self.in_traffic_light {
                let green = tl.can_pass(); // can_pass(&self)
                if !green && vehicle != VehicleType::Ambulance {
                    return false;
                }
            }
        }
        true
    }


    pub fn set_open(&mut self, state: bool) {
        self.is_open = state;
    }

    // Hooks para siguiente paso (colas/prioridades reales)
    pub fn allow_in(&mut self, _thread: &MyThread, _vehicle: VehicleType) {
        // TODO: colas por prioridad, fairness, anti-starvation
    }
    pub fn allow_out(&mut self, _thread: &MyThread, _vehicle: VehicleType) {
        // TODO
    }
}