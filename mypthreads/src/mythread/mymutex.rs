use std::collections::VecDeque;
use crate::mythread::mythread::ThreadId;

pub struct MyMutex {
    pub(crate) owner: Option<ThreadId>,
    pub(crate) waiters: VecDeque<ThreadId>,
}

impl MyMutex {
    pub fn new() -> Self {
        Self { owner: None, waiters: VecDeque::new() }
    }
}
