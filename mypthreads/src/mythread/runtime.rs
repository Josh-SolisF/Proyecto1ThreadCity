// src/mythread/runtime.rs
use std::collections::{HashMap, VecDeque};
use crate::mythread::mythread::{MyThread, ThreadId};
use crate::mythread::thread_state::ThreadState;

pub struct ThreadRuntime {
    threads: HashMap<ThreadId, MyThread>,
    pub(crate) run_queue: VecDeque<ThreadId>,
    next_id: ThreadId,
    current: Option<ThreadId>,

    wait_on: HashMap<ThreadId, Vec<ThreadId>>, // target -> waiters

}

impl ThreadRuntime {
    pub fn new() -> Self {
        Self {
            threads: HashMap::new(),
            run_queue: VecDeque::new(),
            next_id: 1,
            current: None,
            wait_on: HashMap::new(),

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

    /// Reencola un hilo.
    pub fn enqueue(&mut self, tid: ThreadId) {
        // Solo encolamos si no está Terminated.
        if matches!(self.get_state(tid), Some(ThreadState::Ready | ThreadState::Running | ThreadState::Blocked | ThreadState::New)) {
            self.run_queue.push_back(tid);
        }
    }


    /// Ejecuta un “paso” cooperativo: elige, marca Running y fija `current`.
    /// El código de hilo (o tests) deberían llamar luego a `yield/end`.
    pub fn run_once(&mut self) -> Option<ThreadId> {
        let next = self.schedule_next()?;
        self.set_state(next, ThreadState::Running);
        self.current = Some(next);
        Some(next)
    }

    /// Útil cuando el hilo hace yield/end: limpiamos `current`.
    pub fn clear_current(&mut self) {
        self.current = None;
    }

    pub fn current(&self) -> Option<ThreadId> {
        self.current
    }



    pub fn mark_blocked_on(&mut self, waiter: ThreadId, target: ThreadId) {
        self.set_state(waiter, ThreadState::Blocked);
        self.wait_on.entry(target).or_default().push(waiter);

    }

    pub fn on_terminated(&mut self, target: ThreadId) {
        if let Some(waiters) = self.wait_on.remove(&target) {
            for w in waiters {
                self.set_state(w, ThreadState::Ready);
                self.enqueue(w);
            }
        }
    }

    pub fn set_joinable(&mut self, tid: ThreadId, joinable: bool) {
        if let Some(th) = self.threads.get_mut(&tid) {
            th.joinable = joinable;
        }
    }

    pub fn is_joinable(&self, tid: ThreadId) -> Option<bool> {
        self.threads.get(&tid).map(|t| t.joinable)
    }


}