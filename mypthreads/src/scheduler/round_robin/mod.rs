mod tests;

use std::collections::{HashMap, VecDeque};
use crate::mythread::mythread::{MyThread, ThreadId};
use crate::Scheduler;

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
}