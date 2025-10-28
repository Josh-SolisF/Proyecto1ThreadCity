use std::collections::VecDeque;
use crate::mythread::mythread::{MyThread, ThreadId};

pub trait Scheduler {
    /// Encola un hilo listo para ejecutar.
    fn enqueue(&mut self, tid: ThreadId, t: &MyThread);

    /// Devuelve el siguiente hilo a ejecutar (o None si vacío).
    fn pick_next(&mut self) -> Option<ThreadId>;

    /// Eventos para limpiar/actualizar estado interno:
    fn on_block(&mut self, _tid: ThreadId) {}
    fn on_exit(&mut self, _tid: ThreadId) {}

    fn is_empty(&self) -> bool;
}
