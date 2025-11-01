use crate::vehicle::vehicle_type::VehicleType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportPolicy {
    NoVehicles,
    Car,
    Truck,
    Ship,
    AnyVehicle,
}

impl TransportPolicy {
    pub fn can_pass(&self, v: VehicleType) -> bool {
        match self {
            TransportPolicy::NoVehicles => {false},
            TransportPolicy::Car => {matches!(v, VehicleType::CarE| VehicleType::TruckE | VehicleType::AmbulanceE)},
            TransportPolicy::Ship => {matches!(v, VehicleType::ShipE)},
            TransportPolicy::Truck => {matches!(v, VehicleType::TruckE)},
            TransportPolicy::AnyVehicle => {true},
        }
    }
}