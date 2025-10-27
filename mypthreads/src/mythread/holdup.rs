use crate::Scheduler;
use std::os::raw::c_int;
use crate::mythread::mymutex::MyMutex;
use crate::mythread::myruntime::MyTRuntime;
use crate::mythread::mythread::{AnyParam, MyTRoutine, ThreadId};
use crate::mythread::mythreadattr::{MyThreadAttr};

pub struct MyPThread<S: Scheduler> {
    pub(crate) runtime: MyTRuntime<S>,
}

impl<S: Scheduler> MyPThread<S> {
    pub fn new(schedulers: Vec<S>) -> Self {
        let rt = MyTRuntime::new(schedulers);

        Self {
            runtime: rt,
        }
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn my_thread_create(
        &mut self,
        thread: *mut ThreadId,
        attr: *mut MyThreadAttr,
        start_routine: MyTRoutine,
        arg: *mut AnyParam,
    ) -> c_int {
        self.runtime.create(thread, attr, start_routine, arg)
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn my_thread_join(
        &mut self,
        thread: ThreadId,
        ret_val: *mut *mut AnyParam,
    ) -> c_int {
        self.runtime.join(thread, ret_val)
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn my_thread_yield(&mut self) -> c_int {
        self.runtime.save_context();
        self.runtime.schedule_next();
        0

    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn my_thread_end(&mut self, retval: *mut AnyParam) -> c_int {
        self.runtime.end_current(retval)
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn my_thread_detach(&mut self, thread: ThreadId) -> c_int {
        unsafe {
            self.runtime.detach(thread)
        }
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn my_mutex_init(mutex: MyMutex) -> c_int {
        if mutex.nul {  }
    }
}
