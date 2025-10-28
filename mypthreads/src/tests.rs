#[cfg(test)]
mod tests {
    use crate::mythread::mypthread::MyPThread;
    use crate::mythread::mythread::{AnyParam, ThreadId};
    use crate::mythread::mythreadattr::MyThreadAttr;
    use crate::mythread::thread_state::ThreadState;
    use std::ptr;
    use libc::c_int;
    use crate::{RoundRobinScheduler, Scheduler};
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

    fn generate_schedules() -> Vec<Box<dyn Scheduler>> {
        vec![
            Box::new(RoundRobinScheduler),
            Box::new(RoundRobinScheduler),
            Box::new(RoundRobinScheduler),
        ]
    }
    #[test]
    fn test_create_and_join_behaviors() {
        unsafe {
            let mut pth: MyPThread = MyPThread::new(generate_schedules());
            let mut tid: ThreadId = 0;
            let mut my_attr: MyThreadAttr = MyThreadAttr::new(SchedulerType::RoundRobin, 0, 1);
            let mut value_my: i32 = 5;
            let mut retval_my: *mut AnyParam = ptr::null_mut();

            let res = pth.my_thread_create(
                &mut tid,
                &mut my_attr,
                test_thread_function,
                &mut value_my as *mut i32 as *mut AnyParam,
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
            let mut pth: MyPThread = MyPThread::new(generate_schedules());
            const IDS_SIZE: usize = 3;
            let mut ids: [ThreadId; IDS_SIZE] = [1; IDS_SIZE];
            let mut results = [ptr::null_mut(); 3];
            let mut my_attr: MyThreadAttr = MyThreadAttr::new(SchedulerType::RoundRobin, 0, 1);
            let mut value_my: i32 = 5;

            for i in 0..IDS_SIZE {
                let res = pth.my_thread_create(
                    &mut ids[i],
                    &mut my_attr,
                    test_thread_returns_static,
                    &mut value_my as *mut i32 as *mut AnyParam,
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
            let mut pth = MyPThread::new(generate_schedules());

            // Crear dos hilos simples
            let mut tid1: ThreadId = 0;
            let mut tid2: ThreadId = 0;
            let mut attr1 = MyThreadAttr::new(SchedulerType::RoundRobin, 0, 1);
            let mut attr2 = MyThreadAttr::new(SchedulerType::RoundRobin, 0, 1);
            let mut dummy_val: i32 = 0;

            pth.my_thread_create(
                &mut tid1,
                &mut attr1,
                test_thread_returns_static,
                &mut dummy_val as *mut i32 as *mut AnyParam,
            );
            pth.my_thread_create(
                &mut tid2,
                &mut attr2,
                test_thread_returns_static,
                &mut dummy_val as *mut i32 as *mut AnyParam,
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
            let mut pth = MyPThread::new(generate_schedules());
            let mut tid: ThreadId = 0;
            let mut attr = MyThreadAttr::new(SchedulerType::RoundRobin, 0, 1);
            let mut dummy_val: i32 = 0;

            // Crear hilo y ejecutarlo
            pth.my_thread_create(
                &mut tid,
                &mut attr,
                test_thread_returns_static,
                &mut dummy_val as *mut i32 as *mut AnyParam,
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
            let mut pth = MyPThread::new(generate_schedules());
            let mut tid: ThreadId = 0;
            let mut attr = MyThreadAttr::new(SchedulerType::RoundRobin, 0, 1);
            let mut val: i32 = 10;

            // Crear hilo
            pth.my_thread_create(
                &mut tid,
                &mut attr,
                test_thread_returns_static,
                &mut val as *mut i32 as *mut AnyParam,
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
            let mut pth = MyPThread::new(generate_schedules());
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
            let mut pth = MyPThread::new(generate_schedules());
            let mut tid: ThreadId = 0;
            let mut attr = MyThreadAttr::new(SchedulerType::RoundRobin, 0, 1);
            let mut val: i32 = 10;

            pth.my_thread_create(
                &mut tid,
                &mut attr,
                test_thread_returns_static,
                &mut val as *mut i32 as *mut AnyParam,
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
            let mut pth = MyPThread::new(generate_schedules());
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
