use crate::vehicle::vehicle_type::VehicleType;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportPolicy {
    None,
    Car,
    Truck,
    Ship,
    Any,
}

impl TransportPolicy {
    pub fn allows(self, v: VehicleType) -> bool {
        match (self, v) {
            (TransportPolicy::None, _) => false,
            (TransportPolicy::Any, _) => true,
            (TransportPolicy::Car, VehicleType::Car) => true,
            (TransportPolicy::Truck, VehicleType::Truck) => true,
            (TransportPolicy::Ship, VehicleType::Ship) => true,
            _ => false,
        }
    }
}