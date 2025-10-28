use libc::{
    pthread_attr_t,
    pthread_attr_init,
    pthread_attr_destroy,
};

use crate::scheduler::SchedulerType;

pub type PriorityLevel = u8;
pub struct MyThreadAttr {
    inner: pthread_attr_t,
    pub(crate) dead_line: usize,
    pub(crate) priority: PriorityLevel,
    pub(crate) detached: bool,
}

impl MyThreadAttr {
    pub fn new(
        dead_line: usize,
        priority: PriorityLevel,
    ) -> Self {
        unsafe {
            let mut attr: pthread_attr_t = std::mem::zeroed();
            pthread_attr_init(&mut attr);
            Self { inner: attr, dead_line, priority, detached: false }
        }
    }

    pub fn detach(&mut self) {
        self.detached = true;
    }

    /// Devuelve un puntero al pthread_attr_t interno (para pasar a pthread_create)
    pub fn c_pointer(&self) -> *const pthread_attr_t {
        &self.inner
    }
}

impl Drop for MyThreadAttr {
    fn drop(&mut self) {
        unsafe {
            pthread_attr_destroy(&mut self.inner);
        }
    }
}