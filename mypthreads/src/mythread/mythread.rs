use std::os::raw::c_void;
use crate::mythread::mythreadattr::{MyAttr, MyThreadAttr};
use crate::mythread::thread_state::ThreadState;

pub type ThreadId = u64;
pub type AnyParam = c_void;
pub type MyTRoutine =  extern "C" fn(*mut AnyParam) -> *mut AnyParam;

pub struct MyThread {
    pub(crate) id: ThreadId,
    pub(crate) state: ThreadState,
    pub(crate) attr:  MyAttr,
    pub(crate) start_routine: MyTRoutine,
    pub(crate) arg: *mut AnyParam,
    pub(crate) ret_val: *mut AnyParam,
}

impl MyThread {
    pub fn new(id: ThreadId, attr: *const MyAttr, routine: MyTRoutine, arg: *mut AnyParam) -> Self {
        Self {
            id,
            state: ThreadState::New,
            attr: MyAttr { detached: true, stack_size: 1024 },
            start_routine: routine,
            arg,
            ret_val: std::ptr::null_mut(),
        }
    }


    pub fn run(&mut self) {
        // Si ya terminó (por haber llamado my_thread_end dentro de la rutina), no hagas nada
        if self.state == crate::mythread::thread_state::ThreadState::Terminated {
            return;
        }

        // De lo contrario, ejecuta la rutina y termina normalmente
        let result = (self.start_routine)(self.arg);
        self.ret_val = result;
        self.state = crate::mythread::thread_state::ThreadState::Terminated;
    }
}


