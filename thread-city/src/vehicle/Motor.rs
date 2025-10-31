// Estados y cinemática simple de los vehículos sobre/antes del puente

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MoveState {
    Approaching, // acercándose al puente
    Waiting,     // esperando permiso/lock
    Crossing,    // cruzando
    Leaving,     // liberando lock y saliendo
    Done,        // finalizó
}

#[derive(Copy, Clone, Debug)]
pub struct Motion {
    pub pos: f32,        // posición 1D (m)
    pub speed: f32,      // m/s
    pub length: f32,     // longitud del vehículo (m)
    pub direction: i8,   // +1 adelante, -1 atrás
    pub state: MoveState,
}

impl Motion {
    pub fn new(speed: f32, length: f32, direction: i8) -> Self {
        Self {
            pos: 0.0,
            speed,
            length,
            direction: if direction >= 0 { 1 } else { -1 },
            state: MoveState::Approaching,
        }
    }

    pub fn step_approach(&mut self, dt: f32, stop_at: f32) {
        let delta = self.direction as f32 * self.speed * dt;
        let next = self.pos + delta;
        let crossed = (self.direction > 0 && next >= stop_at)
            || (self.direction < 0 && next <= stop_at);
        self.pos = if crossed { stop_at } else { next };
    }

    pub fn step_crossing(&mut self, dt: f32) {
        let delta = self.direction as f32 * self.speed * dt;
        self.pos += delta;
    }
}