#[cfg(test)]
mod tests {
    use crate::mythread::mypthread::MyPThread;
    use crate::mythread::mythread::{AnyParam, ThreadId};
    use crate::mythread::mythreadattr::MyThreadAttr;
    use crate::mythread::thread_state::ThreadState;
    use std::ptr;
    use libc::c_int;
    use crate::mythread::mymutex::MyMutex;
    use crate::scheduler::{SchedulerType};
    use crate::mythread::mypthreadexits::Exits;

    extern "C" fn test_thread_function(arg: *mut AnyParam) -> *mut AnyParam {
        unsafe {
            let value = arg as *mut i32;
            *value += 10;
            value as *mut AnyParam
        }
    }

    extern "C" fn test_thread_returns_static(arg: *mut AnyParam) -> *mut AnyParam {
        println!("Ejecutando hilo con argumento: {:?}", arg);
        let static_value: &'static mut i32 = Box::leak(Box::new(42));
        static_value as *mut i32 as *mut AnyParam
    }


    #[cfg(test)]
    mod tests_change_sched {
        use std::ptr;
        use crate::mythread::mypthread::MyPThread;
        use crate::mythread::mythread::{AnyParam, ThreadId};
        use crate::mythread::mythreadattr::MyThreadAttr;
        use crate::scheduler::scheduler_type::SchedulerType;

        extern "C" fn test_thread_returns_static(_arg: *mut AnyParam) -> *mut AnyParam {
            let static_value: &'static mut i32 = Box::leak(Box::new(42));
            static_value as *mut i32 as *mut AnyParam
        }

        #[test]
        fn test_change_scheduler_to_lottery_and_join() {
            unsafe {
                let mut pth: MyPThread = MyPThread::new();

                let mut tid: ThreadId = 0;
                // deadline = MAX (irrelevante para Lottery), priority=tickets (e.g., 10)
                let mut attr: MyThreadAttr = MyThreadAttr::new(usize::MAX, 10);

                let mut retval: *mut AnyParam = ptr::null_mut();

                // Crear hilo con scheduler por defecto (None -> RoundRobin)
                let r = pth.my_thread_create(
                    &mut tid,
                    &mut attr,
                    test_thread_returns_static,
                    ptr::null_mut(),
                    None,
                );
                assert_eq!(r, 0, "my_thread_create falló");

                // Cambiar el scheduler del hilo a Lottery
                let rc = pth.my_thread_chsched(tid, SchedulerType::Lottery);
                assert_eq!(rc, 0, "my_thread_chsched falló");

                // Unir y verificar retorno
                let j = pth.my_thread_join(tid, &mut retval as *mut *mut AnyParam);
                assert_eq!(j, 0, "my_thread_join falló");

                let rv_i32 = retval as *mut i32;
                assert!(!rv_i32.is_null(), "retval es nulo");
                assert_eq!(*rv_i32, 42, "Valor retornado inesperado");
            }
        }
    }
    #[cfg(test)]
    mod tests_change_sched_order {
        use std::ptr;
        use std::sync::atomic::{AtomicI32, Ordering};
        use crate::mythread::mypthread::MyPThread;
        use crate::mythread::mythread::{AnyParam, ThreadId};
        use crate::mythread::mythreadattr::MyThreadAttr;
        use crate::scheduler::scheduler_type::SchedulerType;

        // 0 = nadie ha corrido aún; 1 = corrió RT primero; 2 = corrió RR primero
        static FIRST_RAN: AtomicI32 = AtomicI32::new(0);

        extern "C" fn mark_rt(_arg: *mut AnyParam) -> *mut AnyParam {
            // marca que el RT corrió primero si FIRST_RAN aún es 0
            FIRST_RAN.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst).ok();
            std::ptr::null_mut()
        }

        extern "C" fn mark_rr(_arg: *mut AnyParam) -> *mut AnyParam {
            // marca que el RR corrió primero si FIRST_RAN aún es 0
            FIRST_RAN.compare_exchange(0, 2, Ordering::SeqCst, Ordering::SeqCst).ok();
            std::ptr::null_mut()
        }

        #[test]
        fn test_change_scheduler_to_realtime_runs_first() {
            unsafe {
                FIRST_RAN.store(0, Ordering::SeqCst);

                let mut pth: MyPThread = MyPThread::new();

                let mut tid_rr: ThreadId = 0;
                let mut tid_rt: ThreadId = 0;

                // RR: deadline irrelevante, priority cualquiera
                let mut attr_rr: MyThreadAttr = MyThreadAttr::new(usize::MAX, 1);
                // RT: deadline más cercano -> debería ejecutarse primero por política global (RT > Lottery > RR)
                let mut attr_rt: MyThreadAttr = MyThreadAttr::new(10, 1);

                // Crear ambos inicialmente con default (RR)
                let r1 = pth.my_thread_create(
                    &mut tid_rr,
                    &mut attr_rr,
                    mark_rr,
                    ptr::null_mut(),
                    None, // RR por defecto
                );
                assert_eq!(r1, 0);

                let r2 = pth.my_thread_create(
                    &mut tid_rt,
                    &mut attr_rt,
                    mark_rt,
                    ptr::null_mut(),
                    None, // arranca RR, lo cambiamos abajo
                );
                assert_eq!(r2, 0);

                // Cambiar el segundo hilo a RealTime (EDF)
                let rc = pth.my_thread_chsched(tid_rt, SchedulerType::RealTime);
                assert_eq!(rc, 0, "my_thread_chsched falló");

                // Para forzar ejecución de ambos, podemos joinear a cualquiera.
                // El scheduler en modo "driver" avanzará y por política debería correr RT primero.
                let mut dummy: *mut AnyParam = ptr::null_mut();

                // Join sobre el RR (esto hará correr primero RT y luego RR)
                let j1 = pth.my_thread_join(tid_rr, &mut dummy as *mut *mut AnyParam);
                assert_eq!(j1, 0);

                // Join el RT (por si no terminó, pero debería ya haber terminado)
                let j2 = pth.my_thread_join(tid_rt, &mut dummy as *mut *mut AnyParam);
                assert_eq!(j2, 0);

                // Verificar que el que corrió primero fue el de RealTime
                let who = FIRST_RAN.load(Ordering::SeqCst);
                assert_eq!(who, 1, "Se esperaba que RealTime corriera primero (who={who})");
            }
        }
    }

    #[test]
    fn test_create_and_join_behaviors() {
        unsafe {
            let mut pth: MyPThread = MyPThread::new();
            let mut tid: ThreadId = 0;

            let mut my_attr: MyThreadAttr = MyThreadAttr::new(usize::MAX, 1);
            let mut value_my: i32 = 5;
            let mut retval_my: *mut AnyParam = ptr::null_mut();

            let res = pth.my_thread_create(
                &mut tid,
                &mut my_attr,
                test_thread_function,
                &mut value_my as *mut i32 as *mut AnyParam,
                Some(SchedulerType::RoundRobin),

            );
            assert_eq!(res, 0, "my_thread_create falló");

            let res_join = pth.my_thread_join(tid, &mut retval_my as *mut *mut AnyParam);
            assert_eq!(res_join, 0, "my_thread_join falló");

            let returned_my = retval_my as *mut i32;

            assert!(!returned_my.is_null(), "El puntero retornado (my_thread_create) es nulo");
            assert_eq!(*returned_my, 15, "Valor incorrecto en my_thread_create (esperado 15)");
            println!("✅ Mypthread funciona correctamente y retorna el valor esperado 15 = {}", *returned_my);
        }
    }

    #[test]
    fn test_multiple_threads() {
        unsafe {
            let mut pth: MyPThread = MyPThread::new();
            const IDS_SIZE: usize = 3;
            let mut ids: [ThreadId; IDS_SIZE] = [1; IDS_SIZE];
            let mut results = [ptr::null_mut(); 3];
            let mut my_attr: MyThreadAttr = MyThreadAttr::new(usize::MAX, 1);
            let mut value_my: i32 = 5;

            for i in 0..IDS_SIZE {
                let res = pth.my_thread_create(
                    &mut ids[i],
                    &mut my_attr,
                    test_thread_returns_static,
                    &mut value_my as *mut i32 as *mut AnyParam,
                    Some(SchedulerType::RoundRobin),

                );
                assert_eq!(res, 0, "my_thread_create falló para hilo {}", i);
            }

            for i in 0..IDS_SIZE {
                let res = pth.my_thread_join(ids[i], &mut results[i] as *mut *mut AnyParam);
                assert_eq!(res, 0, "pthread_join falló para hilo {}", i);

                let ret = results[i] as *mut i32;
                assert_eq!(*ret, 42, "El hilo {:?} no retornó 42", i);
            }
            println!("✅ Todos los hilos retornan el valor esperado.");
        }
    }

    #[test]
    fn test_thread_yield_behavior() {
        unsafe {
            let mut pth = MyPThread::new();

            // Crear dos hilos simples
            let mut tid1: ThreadId = 0;
            let mut tid2: ThreadId = 0;
            let mut attr1: MyThreadAttr = MyThreadAttr::new(usize::MAX, 1);
            let mut attr2: MyThreadAttr = MyThreadAttr::new(usize::MAX, 1);
            let mut dummy_val: i32 = 0;

            pth.my_thread_create(
                &mut tid1,
                &mut attr1,
                test_thread_returns_static,
                &mut dummy_val as *mut i32 as *mut AnyParam,
                Some(SchedulerType::RoundRobin),

            );
            pth.my_thread_create(
                &mut tid2,
                &mut attr2,
                test_thread_returns_static,
                &mut dummy_val as *mut i32 as *mut AnyParam,
                Some(SchedulerType::RoundRobin),

            );

            // Forzar selección inicial
            pth.runtime.schedule_next();
            let before = pth.runtime.get_current().unwrap();
            let state = pth.runtime.get_state(before);
            assert!(state == Some(ThreadState::Running) || state == Some(ThreadState::Terminated));

            // Yield cambia estado actual a Ready y selecciona otro
            let result = pth.my_thread_yield();
            assert_eq!(result, 0, "my_thread_yield debería retornar Ok (0)");

            let after = pth.runtime.get_current().unwrap();
            assert_ne!(before, after, "my_thread_yield no cambió al siguiente hilo");
            let state = pth.runtime.get_state(after);
            assert!(state == Some(ThreadState::Running) || state == Some(ThreadState::Terminated));
            println!(
                "✅ my_thread_yield cambió de hilo correctamente: {:?} → {:?}",
                before, after
            );
        }
    }

    #[test]
    fn test_thread_end_behavior() {
        unsafe {
            let mut pth = MyPThread::new();
            let mut tid: ThreadId = 0;
            let mut attr: MyThreadAttr = MyThreadAttr::new(usize::MAX, 1);
            let mut dummy_val: i32 = 0;

            // Crear hilo y ejecutarlo
            pth.my_thread_create(
                &mut tid,
                &mut attr,
                test_thread_returns_static,
                &mut dummy_val as *mut i32 as *mut AnyParam,
                Some(SchedulerType::RoundRobin),

            );
            pth.runtime.schedule_next();
            assert_eq!(pth.runtime.get_current(), Some(tid));

            // Terminar hilo
            let result = pth.my_thread_end(&mut dummy_val as *mut i32 as *mut AnyParam);
            assert_eq!(result, 0, "my_thread_end no retornó Ok (0)");

            // Verificar que el hilo terminó
            assert_eq!(
                pth.runtime.get_state(tid),
                Some(ThreadState::Terminated),
                "El hilo no fue marcado como Terminated"
            );
            assert!(pth.runtime.get_current().is_none(), "current no fue limpiado");
            println!("✅ my_thread_end marcó correctamente el hilo como Terminated.");
        }
    }


    #[test]
    fn test_thread_detach_behavior() {
        unsafe {
            let mut pth = MyPThread::new();
            let mut tid: ThreadId = 0;
            let mut attr: MyThreadAttr = MyThreadAttr::new(usize::MAX, 1);
            let mut val: i32 = 10;

            // Crear hilo
            pth.my_thread_create(
                &mut tid,
                &mut attr,
                test_thread_returns_static,
                &mut val as *mut i32 as *mut AnyParam,
                Some(SchedulerType::RoundRobin),

            );

            // Detach debería devolver 0
            let res_detach = pth.my_thread_detach(tid);
            assert_eq!(res_detach, 0, "my_thread_detach no retornó Ok (0)");

            // El hilo debe seguir existiendo, pero marcado como detached
            let maybe_thread = pth.runtime.threads.get(&tid);
            assert!(maybe_thread.is_some(), "El hilo debería seguir en la tabla");
            println!("✅ my_thread_detach ejecutado y el hilo sigue activo.");
        }
    }


    #[test]
    fn test_mutex_init_and_destroy() {
        unsafe {
            let mut pth = MyPThread::new();
            let mut mutex = MyMutex::new();

            // Inicializar mutex
            let res_init = pth.my_mutex_init(&mut mutex as *mut MyMutex, std::ptr::null());
            assert_eq!(res_init, 0, "my_mutex_init falló");
            assert!(mutex.initialized, "El mutex no fue marcado como inicializado");

            // Destruir mutex
            let res_destroy = pth.my_mutex_destroy(&mut mutex as *mut MyMutex);
            assert_eq!(res_destroy, 0, "my_mutex_destroy falló");
            assert!(!mutex.initialized, "El mutex debería haberse desinicializado");
            println!("✅ my_mutex_init y my_mutex_destroy ejecutados correctamente.");
        }
    }

    #[test]
    fn test_mutex_lock_and_unlock() {
        unsafe {
            let block_val: c_int = Exits::ThreadBlocked as c_int;
            let mut pth = MyPThread::new();
            let mut tid: ThreadId = 0;
            let mut attr: MyThreadAttr = MyThreadAttr::new(usize::MAX, 1);
            let mut val: i32 = 10;

            pth.my_thread_create(
                &mut tid,
                &mut attr,
                test_thread_returns_static,
                &mut val as *mut i32 as *mut AnyParam,
                Some(SchedulerType::RoundRobin),

            );

            pth.runtime.schedule_next();
            let mut mutex = MyMutex::new();

            // Inicializar mutex
            pth.my_mutex_init(&mut mutex as *mut MyMutex, ptr::null());

            // Bloquear mutex
            let res_lock = pth.my_mutex_lock(&mut mutex as *mut MyMutex);
            assert_eq!(res_lock, block_val, "my_mutex_lock no retornó ThreadBlocked (2)");

            // Asignar manualmente el dueño para la prueba
            mutex.owner = Some(tid);
            mutex.locked.store(true, std::sync::atomic::Ordering::Release);

            // Desbloquear mutex
            let res_unlock = pth.my_mutex_unlock(&mut mutex as *mut MyMutex);
            assert_eq!(res_unlock, 0, "my_mutex_unlock no retornó Ok (0)");
            println!("✅ my_mutex_lock y my_mutex_unlock ejecutados correctamente.");
        }
    }

    #[test]
    fn test_mutex_null_pointer_behavior() {
        unsafe {
            let mut pth = MyPThread::new();
            let null_mutex: *mut MyMutex = std::ptr::null_mut();

            let res_init = pth.my_mutex_init(null_mutex, std::ptr::null());
            assert_ne!(res_init, 0, "my_mutex_init debería fallar con mutex nulo");

            let res_lock = pth.my_mutex_lock(null_mutex);
            assert_ne!(res_lock, 0, "my_mutex_lock debería fallar con mutex nulo");

            let res_unlock = pth.my_mutex_unlock(null_mutex);
            assert_ne!(res_unlock, 0, "my_mutex_unlock debería fallar con mutex nulo");

            println!("✅ Comportamiento de puntero nulo verificado en funciones de mutex.");
        }
    }

}
