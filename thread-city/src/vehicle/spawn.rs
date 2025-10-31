use std::os::raw::c_int;
use mypthreads::mythread::mypthread::MyPThread;
use mypthreads::mythread::mythread::{AnyParam, MyTRoutine, MyThreadAttr, ThreadId};

use crate::cityblock::bridge::Bridge;
use crate::vehicle::vehicle::Vehicle;

use crate::vehicle::vehicle_runner::VehicleCtx;
use crate::vehicle::vehicle_runner::vehicle_routine;

pub fn spawn_vehicle_on_bridge(
    api: &mut MyPThread,
    bridge: &mut Bridge,
    vehicle: &mut Vehicle,  // Debe vivir más que el hilo
    dt_ms: u32,
    bridge_length_m: f32,
) -> c_int {
    // Empaqueta el contexto en heap
    let ctx = Box::new(VehicleCtx {
        bridge: bridge as *mut Bridge,
        vehicle: vehicle as *mut Vehicle,
        dt_ms,
        bridge_length_m,
    });

    let mut tid: ThreadId = 0;
    let arg_ptr = Box::into_raw(ctx) as *mut AnyParam;

    // Construye atributos mínimos (tu MyThreadAttr actual tiene new(dead_line, priority))
    let mut attr = MyThreadAttr::new(0, 0);

    unsafe {
        api.my_thread_create(
            &mut tid as *mut ThreadId,
            &mut attr as *mut MyThreadAttr,
            // 👇 SIN `Some(...)` porque tu API espera MyTRoutine directo
            vehicle_routine as MyTRoutine,
            arg_ptr,
            None, // o Some(SchedulerType::RoundRobin) si lo deseas
        )
    }
}