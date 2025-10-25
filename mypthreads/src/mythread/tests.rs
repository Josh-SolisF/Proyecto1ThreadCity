#[cfg(test)]
mod tests {
    use crate::mythread::mypthread::{my_thread_create, my_thread_join};
    use crate::mythread::myruntime::MyTRuntime;
    use crate::mythread::mythread::{AnyParam, ThreadId};
    use crate::mythread::mythreadattr::MyThreadAttr;
    use crate::mythread::thread_state::ThreadState;
    use libc::{pthread_create, pthread_join};
    use std::ptr;

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

    #[test]
    fn test_compare_create_and_join_behaviors_with_pthread() {
        unsafe {
            let mut my_attr = MyThreadAttr::new();
            my_attr.set_stack_size(64 * 1024);
            my_attr.set_detached(false);
            // -----------------------
            // Caso 1: my_thread_create
            // -----------------------
            let mut tid: ThreadId = 0;
            let mut value_my: i32 = 5;
            let mut retval_my: *mut AnyParam = ptr::null_mut();

            let result_create_my = my_thread_create(
                &mut tid,
                my_attr.as_ptr(),
                test_thread_function,
                &mut value_my as *mut i32 as *mut AnyParam,
            );
            assert_eq!(result_create_my, 0, "my_thread_create falló");

            let result_join_my = my_thread_join(tid, &mut retval_my as *mut *mut AnyParam);
            assert_eq!(result_join_my, 0, "my_thread_join falló");

            let returned_my = retval_my as *mut i32;

            // -----------------------
            // Caso 2: pthread_create
            // -----------------------
            let mut value_p: i32 = 5;
            let mut retval_p: *mut AnyParam = ptr::null_mut();

            // pthread_create usa firma: pthread_create(&mut pthread_t, attr, fn(*mut c_void) -> *mut c_void, arg)
            let result_create_p = pthread_create(
                &mut tid,
                my_attr.as_ptr(),
                test_thread_function,
                &mut value_p as *mut i32 as *mut AnyParam,
            );
            assert_eq!(result_create_p, 0, "pthread_create falló");

            let result_join_p = pthread_join(tid, &mut retval_p as *mut *mut AnyParam);
            assert_eq!(result_join_p, 0, "pthread_join falló");

            let returned_p = retval_p as *mut i32;

            // -----------------------
            // Comparación entre ambas
            // -----------------------
            assert!(!returned_my.is_null(), "El puntero retornado (my_thread_create) es nulo");
            assert_eq!(*returned_my, 15, "Valor incorrecto en my_thread_create (esperado 15)");
            println!("✅ Mypthread funciona correctamente y retorna el valor esperado 15 = {}", *returned_my);

            assert!(!returned_p.is_null(), "El puntero retornado (pthread_create) es nulo");
            assert_eq!(*returned_p, 15, "Valor incorrecto en pthread_create (esperado 15)");
            println!("✅ Pthread acepta correctamente los parametros usados y attr nulos, retorna el valor esperado 15 = {}", *returned_my);

            assert_eq!(
                *returned_my, *returned_p,
                "Los resultados entre my_thread_create y pthread_create no coinciden"
            );

            println!("✅ Ambas implementaciones se comportan de la misma manera.");
        }
    }

    #[test]
    fn test_multiple_threads() {
        unsafe {
            const IDS_SIZE: usize = 3;
            let mut ids: [ThreadId; IDS_SIZE] = [1; IDS_SIZE];
            let mut results = [ptr::null_mut(); 3];

            for i in 0..IDS_SIZE {
                let res = my_thread_create(
                    &mut ids[i],
                    ptr::null(),
                    test_thread_returns_static,
                    ptr::null_mut(),
                );
                assert_eq!(res, 0, "my_thread_create falló para hilo {}", i);
            }

            for i in 0..IDS_SIZE {
                let res = my_thread_join(ids[i], &mut results[i] as *mut *mut AnyParam);
                assert_eq!(res, 0, "pthread_join falló para hilo {}", i);

                let ret = results[i] as *mut i32;
                assert_eq!(*ret, 42, "El hilo {} no retornó 42", i);
            }
            println!("✅ Todos los hilos retornan el valor esperado.");
        }
    }


    #[test]
    fn test_yield_moves_current_to_ready() {
        extern "C" fn dummy_start(_arg: *mut AnyParam) -> *mut AnyParam {
            // No debería ser llamado en este test
            ptr::null_mut()
        }

        let mut rt = MyTRuntime::new();
        let mut tid: ThreadId = 0;
        let attr = MyThreadAttr::new();
        let _ = rt.create(&mut tid, attr.as_ptr(), dummy_start, ptr::null_mut());

        // Simula que el scheduler seleccionó ese hilo como current:
        rt.schedule_next();
        assert_eq!(rt.current, Some(tid));

        // Llamamos yield y verificamos que vuelve a Ready y reencola
        let _ = rt.yield_current();
        assert!(rt.current.is_some()); // si hay sólo un hilo, volverá a ser él mismo en Running
    }
    #[test]
    fn test_end_current_marks_terminated() {
        use crate::mythread::mythread::AnyParam;

        let mut rt = MyTRuntime::new();
        let mut tid: ThreadId = 0;
        let attr = MyThreadAttr::new();

        extern "C" fn start(_a: *mut AnyParam) -> *mut AnyParam {
            // Simula que dentro llama a my_thread_end, pero en este test
            // invocaremos end_current manualmente.
            ptr::null_mut()
        }

        // Crea un hilo
        let _ = rt.create(&mut tid, attr.as_ptr(), start, ptr::null_mut());

        // Simula que es el current
        rt.schedule_next();
        assert_eq!(rt.current, Some(tid));

        // Termina con un valor
        let val = 0x1234usize as *mut AnyParam;
        let rc = rt.end_current(val);
        assert_eq!(rc, 0);

        // Verifica
        let t = rt.threads.get(&tid).unwrap();
        assert_eq!(t.state, ThreadState::Terminated);
        assert_eq!(t.ret_val, val);
    }
}
