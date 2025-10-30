use std::os::raw::c_void;
use libc::pthread_t;
pub use crate::mythread::mythreadattr::MyThreadAttr;
use crate::mythread::thread_state::ThreadState;
use crate::scheduler::SchedulerType;

pub type ThreadId = pthread_t;
pub type AnyParam = c_void;
pub type MyTRoutine =  extern "C" fn(*mut AnyParam) -> *mut AnyParam;

pub struct MyThread {
    pub(crate) id: ThreadId,
    pub(crate) state: ThreadState,
    pub(crate) attr: *mut MyThreadAttr,
    pub(crate) start_routine: MyTRoutine,
    pub(crate) arg: *mut AnyParam,
    pub(crate) ret_val: *mut AnyParam,
    pub(crate) scheduler: SchedulerType,

}

impl MyThread {
    pub fn new(id: ThreadId, attr: *mut MyThreadAttr, routine: MyTRoutine, arg: *mut AnyParam, scheduler: Option<SchedulerType>) -> Self {
        Self {
            id,
            state: ThreadState::New,
            attr,
            start_routine: routine,
            arg,
            ret_val: std::ptr::null_mut(),
            scheduler: scheduler.unwrap_or_default() ,
        }
    }
    
    pub fn run(&mut self) {
        // Si ya terminóno hacemos nada
        if self.state == ThreadState::Terminated {
            return;
        }

        // Si no ejecutamos la rutina y termina normalmente
        let result = (self.start_routine)(self.arg);
        self.ret_val = result;
        self.state = ThreadState::Terminated;
    }

    pub fn id(&self) -> ThreadId {
        self.id
    }

}


