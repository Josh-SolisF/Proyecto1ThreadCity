use std::collections::VecDeque;
use std::os::raw::c_int;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::mythread::mypthreadexits::Exits::{MutexInvalidOwner, MutexInvalidState, MutexLockApproved, MutexLocked, MutexNotInitialized, Ok, UnknownThread};
use crate::mythread::mythread::ThreadId;

pub struct MyMutex {
    pub(crate) initialized: bool,
    pub(crate) owner: Option<ThreadId>,
    pub(crate) locked: AtomicBool,
    pub(crate) wait_queue: VecDeque<ThreadId>,
}

impl MyMutex {
    pub fn is_locked(&self) -> bool {
        todo!()
    }
}

impl MyMutex {
    pub fn new() -> Self {
        Self {
            initialized: false,
            owner: None,
            locked: AtomicBool::new(false),
            wait_queue: VecDeque::new(),
        }
    }
    pub unsafe fn init_mut(&mut self) -> c_int {
        self.locked = AtomicBool::new(false);
        self.owner = None;
        self.wait_queue = VecDeque::new();
        self.initialized = true;

        Ok as c_int
    }


    pub unsafe fn destroy(&mut self) -> c_int {
        if !self.initialized || self.locked.load(Ordering::Acquire) || !self.wait_queue.is_empty() {
            return MutexInvalidState as c_int;
        }
        self.initialized = false;
        self.owner = None;

        Ok as c_int
    }

    pub fn lock(&mut self,tid: ThreadId) -> c_int {
        if !self.initialized {
            return MutexNotInitialized as c_int;
        }

        if self.locked.load(Ordering::Acquire) {
            if !self.wait_queue.contains(&tid) {
                self.wait_queue.push_back(tid);
            }
            return MutexLocked as c_int;
        }
        self.locked.swap(true, Ordering::AcqRel);
        self.owner = Some(tid);

        MutexLockApproved as c_int
    }

    pub fn unlock(&mut self, tid: Option<ThreadId>) -> c_int {
        if tid.is_none() {
            return UnknownThread as c_int;
        }
        if self.owner != tid {
            return MutexInvalidOwner as c_int;
        }
        self.locked.store(false, Ordering::Release);
        self.owner = None;

        Ok as c_int
    }

    pub fn try_lock(&mut self,tid: ThreadId) -> c_int {
        if !self.initialized {
            return MutexNotInitialized as c_int;
        }
        if self.locked.load(Ordering::Acquire) {
            return MutexLocked as c_int;
        }
        self.locked.swap(true, Ordering::AcqRel);
        self.owner = Some(tid);

        Ok as c_int
    }
}




