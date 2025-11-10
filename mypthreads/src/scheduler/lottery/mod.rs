mod tests;

use rand::Rng;
use crate::mythread::mythread::{MyThread, ThreadId};
use super::Scheduler;



pub struct LotteryScheduler {
    entries: Vec<(ThreadId, u32)>, // (tid, tickets)
    rng_state: u64,
}
impl LotteryScheduler {
    pub fn new() -> Self {
        // Semilla
        Self { entries: Vec::new(), rng_state: 0x9E3779B97F4A7C15 }
    }


    #[cfg(test)]
    pub fn with_seed(seed: u64) -> Self {
        Self { entries: Vec::new(), rng_state: seed }
    }


    #[inline]
    fn next_u64(&mut self) -> u64 {
        // splitmix64: https://prng.di.unimi.it/splitmix64.c
        let mut z = self.rng_state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        self.rng_state = z;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    #[inline]
    fn sample_u64_below(&mut self, n: u64) -> u64 {
        // toma la parte alta de (rand * n) para evitar sesgo
        let r = self.next_u64();
        (((r as u128) * (n as u128)) >> 64) as u64
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

    }

    fn on_exit(&mut self, _tid: ThreadId) {  }

    fn is_empty(&self) -> bool { self.entries.is_empty() }
}

