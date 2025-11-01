use crate::vehicle::vehicle_type::VehicleType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportPolicy {
    NoVehicles,
    Car,
    Ship,
    AnyVehicle,
}