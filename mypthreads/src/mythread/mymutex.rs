use std::collections::VecDeque;
use std::os::raw::c_int;
use std::sync::Condvar;
use std::thread::yield_now;
use crate::mythread::mythread::ThreadId;

pub struct MyMutex {
    locked: bool,
    wait_queue: VecDeque<ThreadId>,
}

impl MyMutex {
    pub const fn new() -> Self {
        MyMutex {
            locked: false,
            wait_queue: VecDeque::new(),
        }
    }
    pub fn lock(&mut self, current_thread: ThreadId) {
        if !self.locked {
            self.locked = true;
        } else {
        self.wait_queue.push_back(current_thread);

        }
    }

}