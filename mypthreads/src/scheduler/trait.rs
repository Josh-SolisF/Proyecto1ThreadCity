use std::collections::{HashMap, VecDeque};
use crate::mythread::mythread::{MyThread, ThreadId};

pub trait Scheduler {
    fn schedule(&self, queue: &mut VecDeque<ThreadId> ,threads: &mut HashMap<ThreadId, MyThread>) -> Option<ThreadId>;
}
