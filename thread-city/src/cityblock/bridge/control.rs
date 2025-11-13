use rand::{rng, Rng};
use mypthreads::mythread::mythread::{ThreadId};
use crate::cityblock::bridge::traffic_light::TrafficLight;
use crate::vehicle::vehicle::{PatienceLevel, Vehicle};
use crate::vehicle::vehicle::PatienceLevel::{Critical, Maxed, Low, Starved};
use crate::vehicle::vehicle_type::VehicleType;
use crate::vehicle::vehicle_type::VehicleType::{AmbulanceE, ShipE, TruckE};

pub struct Control {
    pub(crate) in_traffic_light: Option<TrafficLight>,
    pub(crate) out_traffic_light: Option<TrafficLight>,
    pub has_yield: bool,
    pub can_pass_boats: bool,
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
            can_pass_boats: false,
        }
    }
    pub fn without_traffic(
        has_yield: bool,
    ) -> Self {
        Self {
            in_traffic_light: None,
            out_traffic_light: None,
            has_yield,
            can_pass_boats: !has_yield,
        }
    }
    pub fn advance_time(&mut self, frames: usize) {
        if let Some(out_t) = self.out_traffic_light.as_mut() {
            out_t.advance_time(frames);
        }
        if let Some(int_t) = self.in_traffic_light.as_mut() {
            int_t.advance_time(frames);
        }
    }

    pub fn allow_in(&mut self, vehicles: Vec<&Box<dyn Vehicle>>) -> Option<ThreadId> {
        if vehicles.is_empty() {
            return None;
        }
        let temp: Vec<(VehicleType, PatienceLevel)> = vehicles
            .iter()
            .map(|v| (v.get_type().clone(), v.calc_patience()))
            .collect();

        if let Some((i, _)) = temp
            .iter()
            .enumerate()
            .find(|(_, (vtype, patience))| *vtype == TruckE && *patience == Critical)
        {
            return vehicles[i].base().thread_id;
        }
        let ambulances: Vec<(usize, PatienceLevel)> = temp
            .iter()
            .enumerate()
            .filter_map(|(i, (vtype, patience))| {
                if *vtype == AmbulanceE {
                    Some((i, *patience))
                } else {
                    None
                }
            })
            .collect();

        if !ambulances.is_empty() {
            let best = ambulances
                .iter()
                .max_by_key(|(_, p)| match p {
                    Critical => 3,
                    Low => 2,
                    Maxed { .. } => 1,
                    _ => 0
                })
                .unwrap();
            return vehicles[best.0].base().thread_id;
        }

        let mut worst_index: Option<usize> = None;
        let mut worst_value: i32 = -1;

        if !self.has_yield && !self.can_pass_boats && !self.in_traffic_light.clone().unwrap().can_pass() {
            return None;
        }
        for (i, (_, patience)) in temp.iter().enumerate() {
            let score = match patience {
                Critical => 3,
                Low => 2,
                Maxed { .. } => 1,
                _ => 0
            };
            if score > worst_value {
                worst_value = score;
                worst_index = Some(i);
            }
        }

        vehicles[worst_index.unwrap()].base().thread_id
    }
    pub fn allow_out(&mut self, vehicle_type: VehicleType, patience_level: PatienceLevel) -> bool {
        if patience_level == Starved {
            return true
        }
        if !self.has_yield && !self.can_pass_boats {
            if vehicle_type == AmbulanceE || vehicle_type == ShipE { return true }
            return self.out_traffic_light.clone().unwrap().can_pass();
        }
        match vehicle_type {
            AmbulanceE => true,
            _ => {
                let chance = rand::rng().random_range(0.0..1.0);
                match patience_level {
                    Maxed { .. } => chance < 0.6,
                    Low => chance < 0.8,
                    _ => true,
                }
            }
        }
    }
}