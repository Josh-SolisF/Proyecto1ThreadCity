use mypthreads::mythread::mythread::MyThread;
use crate::cityblock::coord::Coord;
use crate::vehicle::Motor::Motion;
// 👈 corrección aquí
use crate::vehicle::vehicle_type::VehicleType;

pub struct Vehicle {
    pub(crate) current_position: Coord,
    pub(crate) vehicle_type: VehicleType,
    pub(crate) destination: Coord,
    pub(crate) speed: u8,        // km/h o m/s según tu modelo (aquí lo casteamos a f32)
    pub(crate) direction: i8,    // +1 hacia adelante, -1 hacia atrás
    pub(crate) length: f32,      // longitud del vehículo (m)
    pub(crate) thread: MyThread,
    pub motion: Motion,
}

// ⚠️ Si no necesitas este duplicado, elimina VehicleStruct.
// Lo dejo comentado para evitar confusión.
/*
pub struct VehicleStruct {
    pub(crate) current_position: Coord,
    pub(crate) vehicle_type: VehicleType,
    pub(crate) destination: Coord,
    pub(crate) speed: u8,
    pub(crate) thread: MyThread,
}
*/

impl Vehicle {
    pub fn new(
        origin: Coord,
        destination: Coord,
        speed: u8,
        vehicle_type: VehicleType,
        direction: i8,
        length: f32,
        thread: MyThread,
    ) -> Vehicle {
        // Motion normaliza el signo de direction internamente
        let motion = Motion::new(speed as f32, length, direction);

        Self {
            current_position: origin,
            vehicle_type,
            destination,
            speed,
            direction,
            length,
            thread,
            motion,
        }
    }

    pub fn change_speed(&mut self, speed: u8) {
        self.speed = speed;
        // si quieres mantener motion en sync:
        // self.motion.speed = speed as f32;
    }

    // helpers
    #[inline]
    pub fn ty(&self) -> VehicleType { self.vehicle_type }   // 👈 corrección aquí

    #[inline]
    pub fn thread(&self) -> &MyThread { &self.thread }
}