#[cfg(test)]
mod tests {
    use crate::mythread::mypthread::{my_thread_create, my_thread_join};
    use crate::mythread::mythread::AnyParam;

    extern "C" fn test_thread_function(arg: *mut AnyParam) -> *mut AnyParam {
        unsafe {
            let value = arg as *mut i32;
            *value += 10;
            value as *mut AnyParam
        }
    }

    #[test]
    fn test_pthread_basic_create_and_join() {
        unsafe {
            let mut thread_id: usize = 0;
            let mut value: i32 = 5;
            let mut retval: *mut AnyParam = std::ptr::null_mut();

            let result_create = my_thread_create(
                &mut thread_id as *mut usize,
                std::ptr::null(), // sin atributos
                test_thread_function,
                &mut value as *mut i32 as *mut AnyParam,
            );
            assert_eq!(result_create, 0, "my_thread_create falló");

            let result_join = my_thread_join(thread_id, &mut retval as *mut *mut AnyParam);
            assert_eq!(result_join, 0, "pthread_join falló");

            let returned = retval as *mut i32;
            assert!(!returned.is_null(), "El puntero de retorno es nulo");

            assert_eq!(*returned, 15, "El valor retornado no coincide (esperado 15)");
        }
    }

    extern "C" fn test_thread_returns_static(arg: *mut AnyParam) -> *mut AnyParam {
        println!("Ejecutando hilo con argumento: {:?}", arg);
        let static_value: &'static mut i32 = Box::leak(Box::new(42));
        static_value as *mut i32 as *mut AnyParam
    }

    #[test]
    fn test_multiple_threads() {
        unsafe {
            let mut ids = [0usize; 3];
            let mut results = [std::ptr::null_mut(); 3];

            for i in 0..3 {
                let res = my_thread_create(
                    &mut ids[i] as *mut usize,
                    std::ptr::null(),
                    test_thread_returns_static,
                    std::ptr::null_mut(),
                );
                assert_eq!(res, 0, "my_thread_create falló para hilo {}", i);
            }

            for i in 0..3 {
                let res = my_thread_join(ids[i], &mut results[i] as *mut *mut AnyParam);
                assert_eq!(res, 0, "pthread_join falló para hilo {}", i);

                let ret = results[i] as *mut i32;
                assert_eq!(*ret, 42, "El hilo {} no retornó 42", i);
            }
        }
    }
}
