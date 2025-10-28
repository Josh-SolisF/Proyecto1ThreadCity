mod tests;

use std::cmp::Reverse;
use rand::Rng;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use crate::mythread::mythread::{MyThread, ThreadId};
use crate::mythread::mythreadattr::PriorityLevel;
use crate::Scheduler;

/// 0 = prioridad más, 255 = menos.

pub struct LotteryScheduler {
    entries: Vec<(ThreadId, u32)>, // (tid, tickets)
    rng_state: u64,
}
impl LotteryScheduler {
    pub fn new() -> Self {
        // Semilla 
        Self { entries: Vec::new(), rng_state: 0x9E3779B97F4A7C15 }
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        // LCG sencillo
        self.rng_state = self.rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        self.rng_state
    }
}

impl Scheduler for LotteryScheduler {
    fn enqueue(&mut self, tid: ThreadId, t: &MyThread) {
        let tickets = unsafe {
            // Usa priority como tickets (mín 1)
            let p = (*t.attr).priority as u32;
            p.max(1)
        };
        self.entries.push((tid, tickets));
    }

    fn pick_next(&mut self) -> Option<ThreadId> {
        if self.entries.is_empty() { return None; }

        let total: u64 = self.entries.iter().map(|&(_, tk)| tk as u64).sum();
        if total == 0 { return None; }

        let r = (self.next_u64() % total) as u64;
        let mut acc = 0u64;
        let mut idx = 0usize;

        for (i, &(_, tk)) in self.entries.iter().enumerate() {
            acc += tk as u64;
            if r < acc {
                idx = i;
                break;
            }
        }
        let (tid, _) = self.entries.swap_remove(idx);
        Some(tid)
    }

    fn on_block(&mut self, _tid: ThreadId) {
        // En esta versión mínima no mantenemos mapa para quitar selectivamente.
        // Ver necesitamos quitar al bloquear, si es así conviene cambiar a HashMap<tid, tickets>
        // y una lista separada de índices por pick aleatorio.
    }

    fn on_exit(&mut self, _tid: ThreadId) { /* idem comentario */ }

    fn is_empty(&self) -> bool { self.entries.is_empty() }
}

 