mod tests;

use std::collections::VecDeque;
use crate::mythread::mythread::{MyThread, ThreadId};
use crate::Scheduler;



pub struct RRScheduler {
    q: VecDeque<ThreadId>,
}

impl RRScheduler {
    pub fn new() -> Self { Self { q: VecDeque::new() } }
}

impl Scheduler for RRScheduler {
    fn enqueue(&mut self, tid: ThreadId, _t: &MyThread) {
        self.q.push_back(tid);
    }
    fn pick_next(&mut self) -> Option<ThreadId> {
        self.q.pop_front()
    }
    fn is_empty(&self) -> bool { self.q.is_empty() }
}


