#[cfg(test)]
mod tests {
    use super::*;
    use std::os::raw::c_void;
    use std::sync::atomic::AtomicBool;

    use crate::cityblock::block::BlockBase;
    use crate::cityblock::block_type::BlockType;
    use crate::cityblock::coord::Coord;
    use crate::cityblock::transport_policy::TransportPolicy;
    use crate::cityblock::bridge::control::Control;
    use crate::vehicle::vehicle::Vehicle;
    use crate::vehicle::vehicle_type::VehicleType;

    use mypthreads::mythread::mymutex::MyMutex;
    use mypthreads::mythread::mythread::{MyThread, ThreadId, MyThreadAttr};
    use crate::cityblock::bridge::Bridge;
    // ---------- Helpers de construcción ----------

    // Rutina vacía con la firma C que requiere MyThread
    extern "C" fn noop(_: *mut c_void) -> *mut c_void {
        std::ptr::null_mut()
    }

    // Crea un MyThread con un ID dado (no ejecutamos nada real en los tests)
    fn mk_thread_with_id(id: ThreadId) -> MyThread {
        let attr: *mut MyThreadAttr = std::ptr::null_mut();
        let arg: *mut c_void = std::ptr::null_mut();
        MyThread::new(id, attr, noop, arg, None)
    }

    fn mk_thread() -> MyThread {
        mk_thread_with_id(1 as ThreadId)
    }

    // Crea un BlockBase de tipo Bridge. Como los campos son pub(crate), podemos usar literal.
    fn mk_block_bridge() -> BlockBase {
        BlockBase {
            id: 100,
            position: Coord::new(5, 5),
            policy: TransportPolicy::Any,
            occupied: AtomicBool::new(false),
            block_type: BlockType::Bridge,
        }
    }

    // Crea un mutex y lo inicializa (requerido por tu implementación)
    fn mk_mutex() -> MyMutex {
        let mut m = MyMutex::new();
        unsafe { let _ = m.init_mut(); }
        m
    }

    // Construye un vehículo con tipo y un ThreadId específico (para ser owner del mutex)
    fn mk_vehicle(ty: VehicleType, tid: ThreadId) -> Vehicle {
        Vehicle::new(
            Coord::new(0, 0),
            Coord::new(10, 0),

            1,                  // speed: u8
            ty,                 // VehicleType
            1_i8,               // direction: i8  (1 hacia adelante, -1 hacia atrás)
            4.5_f32,            // length: f32    (ej.: 4.5 m para un auto)

            mk_thread_with_id(tid),
        )
    }

    // ---------- Tests ----------

    #[test]
    fn ask_pass_with_open_without_semaphore() {
        let block = mk_block_bridge();
        let control = Control::without_traffic(
            /*has_yield*/ false,
            /*is_open*/  true,
            /*in_policy*/  Some(TransportPolicy::Any),
            /*out_policy*/ Some(TransportPolicy::Any),
        );
        let mutex = mk_mutex();
        let bridge = Bridge::new(block, control, mutex);

        let car = mk_vehicle(VehicleType::Car, 11);
        assert!(
            bridge.ask_pass(car.thread(), car.ty()),
            "Debe permitir paso cuando está abierto sin semáforo y política Any"
        );
    }

    #[test]
    fn enter_then_exit_respects_mutex_ownership() {
        let block = mk_block_bridge();
        let control = Control::without_traffic(false, true, Some(TransportPolicy::Any), None);
        let mutex = mk_mutex();
        let mut bridge = Bridge::new(block, control, mutex);

        let car_a = mk_vehicle(VehicleType::Car, 21);
        let car_b = mk_vehicle(VehicleType::Car, 22);

        // Car A entra (toma el lock)
        assert!(bridge.enter_bridge(&car_a), "Car A debería tomar el lock");

        // Car B no puede liberar el lock (no es el owner)
        assert!(
            !bridge.exit_bridge(&car_b),
            "Car B NO es dueño: unlock debería fallar y retornar false"
        );

        // Car A (dueño) sí puede liberar
        assert!(bridge.exit_bridge(&car_a), "Car A (dueño) libera el lock");
    }

    #[test]
    fn mutex_blocks_second_entry_until_first_exits() {
        let block = mk_block_bridge();
        let control = Control::without_traffic(false, true, Some(TransportPolicy::Any), None);
        let mutex = mk_mutex();
        let mut bridge = Bridge::new(block, control, mutex);

        let car = mk_vehicle(VehicleType::Car, 31);

        // Primer ingreso
        assert!(bridge.enter_bridge(&car));
        // Mientras el lock está tomado, ask_pass puede seguir diciendo true/false según tu implementación,
        // pero la entrada real debe fallar al no poder tomar el lock:
        let car2 = mk_vehicle(VehicleType::Car, 32);
        let enter2 = bridge.enter_bridge(&car2);
        assert!(
            !enter2,
            "Segundo vehículo no debería poder entrar mientras el mutex está tomado"
        );

        // Libera y ahora sí debería poder
        assert!(bridge.exit_bridge(&car));
        let enter2_after = bridge.enter_bridge(&car2);
        assert!(enter2_after, "Al liberar el lock, el segundo vehículo ya puede entrar");
        assert!(bridge.exit_bridge(&car2));
    }

    #[test]
    fn policy_denies_truck_when_only_car_allowed() {
        let block = mk_block_bridge();
        let control = Control::without_traffic(
            false, true,
            Some(TransportPolicy::Car), // Solo Car permitido
            None,
        );
        let mutex = mk_mutex();
        let bridge = Bridge::new(block, control, mutex);

        let car = mk_vehicle(VehicleType::Car, 41);
        let truck = mk_vehicle(VehicleType::Truck, 42);

        assert!(
            bridge.ask_pass(car.thread(), car.ty()),
            "Car debe estar permitido por política Car"
        );
        assert!(
            !bridge.ask_pass(truck.thread(), truck.ty()),
            "Truck debe ser denegado por política Car"
        );
    }

    #[test]
    fn emergency_bypasses_red_light() {
        let block = mk_block_bridge();
        // Semáforo activo y política Any
        let mut control = Control::with_traffic(
            100, 100,
            false,
            true,
            Some(TransportPolicy::Any),
            Some(TransportPolicy::Any),
        );
        let mutex = mk_mutex();
        let mut bridge = Bridge::new(block, control, mutex);

        // Forzamos a ROJO (toggle)
        bridge.control.advance_time(100);

        let amb = mk_vehicle(VehicleType::Ambulance, 51);
        assert!(
            bridge.ask_pass(amb.thread(), amb.ty()),
            "Ambulancia debe pasar aun con semáforo en rojo"
        );
        assert!(bridge.enter_bridge(&amb), "Ambulancia debe poder tomar el lock");
        assert!(bridge.exit_bridge(&amb));
    }

    #[test]
    fn car_blocked_by_red_light() {
        let block = mk_block_bridge();
        let mut control = Control::with_traffic(
            100, 100, false, true,
            Some(TransportPolicy::Any),
            Some(TransportPolicy::Any),
        );
        let mutex = mk_mutex();
        let mut bridge = Bridge::new(block, control, mutex);

        let car = mk_vehicle(VehicleType::Car, 61);

        // Al inicio: verde
        assert!(bridge.ask_pass(car.thread(), car.ty()));
        assert!(bridge.enter_bridge(&car));
        assert!(bridge.exit_bridge(&car));

        // Cambiamos a rojo
        bridge.control.advance_time(100);
        assert!(
            !bridge.ask_pass(car.thread(), car.ty()),
            "Car debe quedar bloqueado por el semáforo en rojo"
        );
    }

    #[test]
    fn open_and_close_bridge_requires_owner_caller() {
        let block = mk_block_bridge();
        let control = Control::without_traffic(false, true, Some(TransportPolicy::Any), None);
        let mutex = mk_mutex();
        let mut bridge = Bridge::new(block, control, mutex);

        let operator = mk_thread(); // quien abre/cierra el puente
        let car = mk_vehicle(VehicleType::Car, 71);

        // Operador abre (toma lock) y deshabilita entrada de vehículos
        assert!(bridge.open_bridge(&operator));
        assert!(
            !bridge.enter_bridge(&car),
            "Mientras el operador tiene el lock y el puente está cerrado a vehículos, el auto no debe entrar"
        );

        // Operador cierra y libera lock
        assert!(bridge.close_bridge(&operator));
        assert!(bridge.enter_bridge(&car), "Tras cerrar, el auto debe poder entrar");
        assert!(bridge.exit_bridge(&car));
    }
}