use std::os::raw::c_void;
use std::ptr::null_mut;
use crate::mythread::mythreadattr::MyThreadAttr;
// src/mythread/mythread.rs
use crate::mythread::thread_state::ThreadState;

pub type ThreadId = usize;
pub type AnyParam = c_void;
pub type MyTRoutine =  extern "C" fn(*mut AnyParam) -> *mut AnyParam;

pub struct MyThread {

    pub(crate) id: ThreadId,
    pub(crate) state: ThreadState,
    pub(crate) attr: MyThreadAttr,
    pub(crate) start_routine: MyTRoutine,
    pub(crate) arg: *mut AnyParam,
    pub(crate) ret_val: *mut AnyParam,
    pub(crate) joinable: bool,
}

impl MyThread {
    pub fn new(id: ThreadId, attr: MyThreadAttr, routine: MyTRoutine, arg: *mut AnyParam) -> Self {
        Self {
            id,
            state: ThreadState::New,
            attr,
            start_routine: routine,
            arg,
            ret_val: std::ptr::null_mut(),
            joinable: true,
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


