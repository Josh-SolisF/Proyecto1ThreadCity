// src/mythread/runtime.rs
use std::collections::{HashMap, VecDeque};
use crate::mythread::mythread::{MyThread, ThreadId};
use crate::mythread::thread_state::ThreadState;

pub struct ThreadRuntime {
    threads: HashMap<ThreadId, MyThread>,
    run_queue: VecDeque<ThreadId>,
    next_id: ThreadId,
}

impl ThreadRuntime {
    pub fn new() -> Self {
        Self {
            threads: HashMap::new(),
            run_queue: VecDeque::new(),
            next_id: 1,
        }
    }

    /// Crea un hilo en estado New, lo pasa a Ready y lo encola.
    pub fn spawn(&mut self, name: &str, entry: Option<fn()>) -> ThreadId {
        let id = self.next_id;
        self.next_id += 1;

        let mut th = MyThread::new(id, name, entry);
        th.state = ThreadState::Ready; // pasa a listo para correr

        self.threads.insert(id, th);
        self.run_queue.push_back(id);
        id
    }

    /// Selecciona el siguiente hilo en la cola (Round Robin básico).
    pub fn schedule_next(&mut self) -> Option<ThreadId> {
        self.run_queue.pop_front()
    }

    /// Cambia el estado del hilo si existe.
    pub fn set_state(&mut self, tid: ThreadId, st: ThreadState) {
        if let Some(th) = self.threads.get_mut(&tid) {
            th.state = st;
        }
    }

    /// Devuelve el estado actual (útil para tests).
    pub fn get_state(&self, tid: ThreadId) -> Option<ThreadState> {
        self.threads.get(&tid).map(|t| t.state)
    }

    /// Reencola un hilo (p. ej. tras yield).
    pub fn enqueue(&mut self, tid: ThreadId) {
        // Solo encolamos si no está Terminated.
        if matches!(self.get_state(tid), Some(ThreadState::Ready | ThreadState::Running | ThreadState::Blocked | ThreadState::New)) {
            self.run_queue.push_back(tid);
        }
    }

    /// Acceso de depuración opcional.
    pub fn len(&self) -> usize {
        self.threads.len()
    }
}