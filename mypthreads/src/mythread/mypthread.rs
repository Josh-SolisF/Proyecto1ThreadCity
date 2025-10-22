use crate::mythread::runtime::ThreadRuntime;
use crate::mythread::mythread::ThreadId;
use crate::mythread::thread_state::ThreadState;

/// Crea un hilo listo para ejecutar "Ready". .
pub fn my_thread_create(rt: &mut ThreadRuntime, name: &str, entry: Option<fn()>) -> ThreadId {
    rt.spawn(name, entry)
}

/// El hilo actual cede la CPU .
/// Convención: quien llama pasa su propio `current_tid`.
pub fn my_thread_yield(rt: &mut ThreadRuntime, current_tid: ThreadId) {
    // Marcar Ready y reencolar. (Suponemos estaba Running.)
    rt.set_state(current_tid, ThreadState::Ready);
    rt.enqueue(current_tid);
    // aqui iria `schedule_next()` y “cambiará” de hilo.
}

/// El hilo actual termina su ejecución.
pub fn my_thread_end(rt: &mut ThreadRuntime, current_tid: ThreadId) {
    rt.set_state(current_tid, ThreadState::Terminated);
    
}