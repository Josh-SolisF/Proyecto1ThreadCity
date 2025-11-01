use mypthreads::mythread::mythread::MyThread;
use crate::cityblock::coord::Coord;
use crate::vehicle::vehicle_type::VehicleType;

pub struct Vehicle {
    pub(crate) current_position: Coord,
    pub(crate) vehicle_type: VehicleType,
    pub(crate) destination: Coord,
    pub(crate) speed: u8,
    pub(crate) thread: MyThread,
}


pub struct VehicleStruct {
    pub(crate) current_position: Coord,
    pub(crate) vehicle_type: VehicleType,
    pub(crate) destination: Coord,
    pub(crate) speed: u8,
    pub(crate) thread: MyThread,
}

impl Vehicle {
    pub fn new(origin: Coord, destination: Coord, speed: u8, vehicle_type: VehicleType, thread: MyThread) -> Vehicle {
        Self {
            vehicle_type,
            current_position: origin,
            destination,
            speed,
            thread,
        }
    }
    
    pub fn change_speed(&mut self, speed: u8) {
        self.speed = speed;
    }


    // helpers
    pub fn ty(&self) -> VehicleType { self.vehicle_type }
    pub fn thread(&self) -> &MyThread { &self.thread }

}