// src/mythread/mythread.rs
use crate::mythread::thread_state::ThreadState;

pub type ThreadId = u32;

pub struct MyThread {
    pub id: ThreadId,
    pub name: String,
    pub state: ThreadState,
    // Placeholder: más adelante guardar la función a ejecutar:
    pub entry: Option<fn()>, // por ahora, función simple para no complicar
    pub joinable: bool
}

impl MyThread {
    pub fn new(id: ThreadId, name: &str, entry: Option<fn()>) -> Self {
        Self {
            id,
            name: name.to_string(),
            state: ThreadState::New,
            entry,
            joinable: true,
        }
    }
}