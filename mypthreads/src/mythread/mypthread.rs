use crate::Scheduler;
use std::os::raw::c_int;
use crate::mythread::mutexlockkind::MyMutexAttr;
use crate::mythread::mymutex::{MyMutex};
use crate::mythread::mypthreadexits::Exits::{Ok, MutexNotInitialized, NullMutex, ThreadBlocked, CurrentIsEmpty};
use crate::mythread::myruntime::MyTRuntime;
use crate::mythread::mythread::{AnyParam, MyTRoutine, ThreadId};
use crate::mythread::mythreadattr::{MyThreadAttr};

pub struct MyPThread {
    pub(crate) runtime: MyTRuntime,
}

impl MyPThread {
    pub fn new(schedulers: Vec<Box<dyn Scheduler>>) -> Self {
        let rt = MyTRuntime::new(schedulers);
        Self {
            runtime: rt,
        }
    }


    pub unsafe extern "C" fn my_thread_create(
        &mut self,
        thread: *mut ThreadId,
        attr: *mut MyThreadAttr,
        start_routine: MyTRoutine,
        arg: *mut AnyParam,
    ) -> c_int {
        self.runtime.create(thread, attr, start_routine, arg)
    }

    pub unsafe extern "C" fn my_thread_join(
        &mut self,
        thread: ThreadId,
        ret_val: *mut *mut AnyParam,
    ) -> c_int {
        self.runtime.join(thread, ret_val)
    }

    pub unsafe extern "C" fn my_thread_yield(&mut self) -> c_int {
        self.runtime.save_context();
        self.runtime.schedule_next();

        Ok as c_int
    }

    pub unsafe extern "C" fn my_thread_end(&mut self, retval: *mut AnyParam) -> c_int {
        self.runtime.end_current(retval)
    }

    pub unsafe extern "C" fn my_thread_detach(&mut self, thread: ThreadId) -> c_int {
        unsafe {
            self.runtime.detach(thread)
        }
    }

    pub unsafe extern "C" fn my_mutex_init(&mut self, mutex: *mut MyMutex, attr: *const MyMutexAttr) -> c_int {
        if mutex.is_null() {
            return MutexNotInitialized as c_int;
        }
        unsafe {
            // Attr lo ignoramos porque no usaremos distintos tipos de mutex
            (*mutex).init_mut()
        }
    }

    pub unsafe extern "C" fn my_mutex_destroy(&mut self, mutex: *mut MyMutex) -> c_int {
        unsafe {
            (*mutex).destroy()
        }
    }

    pub unsafe extern "C" fn my_mutex_lock(&mut self, mutex: *mut MyMutex) -> c_int {
        if mutex.is_null() {
            return NullMutex as c_int;
        }
        unsafe {
            if let Some(tid) = self.runtime.get_current() {
                (*mutex).lock(tid);
            } else {
                return CurrentIsEmpty as c_int;
            }
        }
        ThreadBlocked as c_int
    }

    pub unsafe extern "C" fn my_mutex_unlock(&mut self, mutex: *mut MyMutex) -> c_int {
        if mutex.is_null() {
            return NullMutex as c_int;
        }
        unsafe {
            (*mutex).unlock(self.runtime.get_current())
        }
    }
}
