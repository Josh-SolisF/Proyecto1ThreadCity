use std::fmt::Debug;

pub type ThreadId = u64;

/// El Scheduler:
/// - Llama a `on_ready(tid)` cuando un hilo pasa a **Ready**.
/// - Llama a `pick_next()` para obtener el próximo **Ready** a ejecutar.
/// - Notifica `on_blocked(tid)` cuando un hilo pasa a **Blocked**.
/// - Notifica `on_exit(tid)` cuando un hilo termina (**Terminated**).
///

pub trait Scheduler: Debug + Send{

    fn on_ready(&mut self, tid: ThreadId);

    /// Devuelve el próximo hilo listo (o None si no hay).
    fn pick_next(&mut self) -> Option<ThreadId>;

    /// Notificación: `tid` quedó bloqueado (no está listo).
    fn on_blocked(&mut self, _tid: ThreadId) {}

    /// Notificación: `tid` finalizó (no volverá a estar listo).
    fn on_exit(&mut self, _tid: ThreadId) {}

    /// Identificador humano de la política.
    fn name(&self) -> &'static str;

}