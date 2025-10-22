use std::sync::{Arc, Mutex};
use crate::mythread::mythread::{MyThread, ThreadId};
use crate::mythread::thread_state::{ThreadRuntime, ThreadState};
use crate::scheduler::SchedulerType;

pub trait MyPThreads {

    /*
    pub struct MyMutex {
    pub locked: Mutex<bool>,
}

impl MyMutex {
    pub fn new() -> Self {
        Self {
            locked: Mutex::new(false),
        }
    }
}
    */
    fn my_thread_create(
        rt: &mut ThreadRuntime,
        name: &str,
        sched: SchedulerType,
        entry: fn(),
        tickets: Option<u32>,
    ) -> ThreadId {
        rt.spawn(name, sched, entry, tickets)
    }

    fn my_thread_end() -> ThreadState {
        ThreadState::Exit
    }

    fn my_thread_yield() -> ThreadState {
        ThreadState::Yield
    }

    fn my_thread_detach() -> ThreadState {
        // En POSIX, detach libera recursos del hilo sin esperar join
        ThreadState::Success
    }

    fn my_thread_join() -> ThreadState {
        // Aquí se esperaría a que el hilo termine
        ThreadState::Success
    }

    fn my_mutex_init() -> (ThreadState, MyMutex) {
        (ThreadState::Success, MyMutex::new())
    }

    fn my_mutex_destroy(_mutex: MyMutex) -> ThreadState {
        ThreadState::Success
    }

    fn my_mutex_lock(mutex: &MyMutex) -> ThreadState {
        let mut lock = mutex.locked.lock().unwrap();
        while *lock {
            // Esperar hasta que se libere
        }
        *lock = true;
        ThreadState::Success
    }

    fn my_mutex_unlock(mutex: &MyMutex) -> ThreadState {
        let mut lock = mutex.locked.lock().unwrap();
        *lock = false;
        ThreadState::Success
    }

    fn my_mutex_trylock(mutex: &MyMutex) -> ThreadState {
        match mutex.locked.try_lock() {
            Ok(mut lock) => {
                if !*lock {
                    *lock = true;
                    ThreadState::Success
                } else {
                    ThreadState::Blocked
                }
            }
            Err(TryLockError::WouldBlock) => ThreadState::Blocked,
            Err(e) => ThreadState::Error(format!("{:?}", e)),
        }

} }



