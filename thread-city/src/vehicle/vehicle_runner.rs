use crate::cityblock::bridge::Bridge;
use crate::vehicle::vehicle::Vehicle;
use mypthreads::mythread::mythread::AnyParam;
use crate::vehicle::Motor::MoveState;

/// Contexto que se pasa a la rutina del hilo del vehículo.
#[derive(Copy, Clone, Debug)]
pub struct VehicleCtx {
    pub bridge: *mut Bridge,
    pub vehicle: *mut Vehicle,
    pub dt_ms: u32,
    pub bridge_length_m: f32,
}

// Yield cooperativo (por ahora no-op). Si ya tienes una API para yield,
// llámala aquí; por ejemplo `unsafe { api.my_thread_yield() }` vía un shim.
#[inline]
fn yield_coop(_v: &Vehicle) {
    // no-op; reemplaza si ya expones yield cooperativo
}

unsafe fn get<'a, T>(p: *mut T) -> &'a mut T { &mut *p }

/// Ajusta la firma a tu alias actual:
/// En tu código, `pub type MyTRoutine = extern "C" fn(*mut AnyParam) -> *mut AnyParam;`
pub extern "C" fn vehicle_routine(arg: *mut AnyParam) -> *mut AnyParam {
    // Cast del contexto
    let ctx = unsafe { get::<VehicleCtx>(arg as *mut VehicleCtx) };
    let bridge = unsafe { get::<Bridge>(ctx.bridge) };
    let v = unsafe { get::<Vehicle>(ctx.vehicle) };

    let dt = (ctx.dt_ms as f32) / 1000.0;
    let half_vehicle = 0.5 * v.motion.length;

    // Eje 1D: entrada en 0.0, salida en bridge_length_m
    let entry = if v.motion.direction > 0 { 0.0 } else { ctx.bridge_length_m };
    let exit  = if v.motion.direction > 0 { ctx.bridge_length_m } else { 0.0 };

    loop {
        match v.motion.state {
            MoveState::Approaching => {
                v.motion.step_approach(dt, entry);
                let front = v.motion.pos + (v.motion.direction as f32) * half_vehicle;
                let reached_entry =
                    (v.motion.direction > 0 && front >= entry) ||
                        (v.motion.direction < 0 && front <= entry);
                if reached_entry {
                    v.motion.state = MoveState::Waiting;
                }
                yield_coop(&*v);
            }

            MoveState::Waiting => {
                // Intenta entrar (no bloqueante) usando Bridge::enter_bridge
                let can = bridge.enter_bridge(&*v);
                if can {
                    v.motion.state = MoveState::Crossing;
                } else {
                    yield_coop(&*v);
                }
            }

            MoveState::Crossing => {
                v.motion.step_crossing(dt);
                let front = v.motion.pos + (v.motion.direction as f32) * half_vehicle;
                let passed_exit =
                    (v.motion.direction > 0 && front >= exit) ||
                        (v.motion.direction < 0 && front <= exit);
                if passed_exit {
                    v.motion.state = MoveState::Leaving;
                }
                yield_coop(&*v);
            }

            MoveState::Leaving => {
                let _ = bridge.exit_bridge(&*v);
                v.motion.state = MoveState::Done;
            }

            MoveState::Done => break,
        }
    }

    core::ptr::null_mut()
}