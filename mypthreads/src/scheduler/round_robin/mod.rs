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


/*
pub struct RoundRobinScheduler;
impl Scheduler for RoundRobinScheduler {
    fn schedule(&self, queue: &mut VecDeque<ThreadId>, threads: &mut HashMap<ThreadId, MyThread>) -> Option<ThreadId> {
        // TODO: Debe filtrar primero cuales de los Thread son pertenecientes al respectivo Scheduler
        // TODO: Despúes debe aplicar su selección según el tipo de scheduler y retornar el tid favorito.
        if let Some(tid) = queue.get(0) {
            if let Some(thread) = threads.get(&tid) {
               Some(*tid)
            } else { None }
        } else { None }
    }
}*/