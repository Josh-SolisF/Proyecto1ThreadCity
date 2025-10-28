mod tests;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use crate::mythread::mythread::{MyThread, ThreadId};
use crate::Scheduler;

pub struct RealTimeScheduler {
    heap: BinaryHeap<Reverse<(usize, ThreadId)>>, // (deadline, tid)
}
impl RealTimeScheduler { pub fn new() -> Self { Self { heap: BinaryHeap::new() } } }

impl Scheduler for RealTimeScheduler {
    fn enqueue(&mut self, tid: ThreadId, t: &MyThread) {
        let dl = unsafe { (*t.attr).dead_line };
        self.heap.push(Reverse((dl, tid)));
    }
    fn pick_next(&mut self) -> Option<ThreadId> {
        self.heap.pop().map(|Reverse((_dl, tid))| tid)
    }
    fn is_empty(&self) -> bool { self.heap.is_empty() }
}

