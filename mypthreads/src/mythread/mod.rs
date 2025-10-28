use crate::mythread::mythread::{MyThread, ThreadId};

pub mod mypthread;
pub mod mythread;
pub(crate) mod mymutex;
pub mod thread_state;
mod myruntime;
pub(crate) mod mythreadattr;

mod mutexlockkind;
pub mod mypthreadexits;

pub trait Scheduler {
    fn enqueue(&mut self, tid: ThreadId, t: &MyThread);
    fn pick_next(&mut self) -> Option<ThreadId>;
    fn on_block(&mut self, _tid: ThreadId) {}
    fn on_exit(&mut self, _tid: ThreadId) {}
    fn is_empty(&self) -> bool;
}
