mod tests;

use std::collections::{HashMap, VecDeque};
use crate::mythread::mythread::{MyThread, ThreadId};
use crate::Scheduler;

pub struct LotteryScheduler;
impl Scheduler for LotteryScheduler {
    fn schedule(&self, queue: &mut VecDeque<ThreadId>, threads: &mut HashMap<ThreadId, MyThread>) -> Option<ThreadId> {
        todo!()
    }
}