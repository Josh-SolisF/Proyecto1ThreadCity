use crate::mythread::mymutex::MyMutex;
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
    rt.clear_current();
    // aqui iria `schedule_next()` y “cambiará” de hilo.
}

/// El hilo actual termina su ejecución.
pub fn my_thread_end(rt: &mut ThreadRuntime, current_tid: ThreadId) {
    rt.set_state(current_tid, ThreadState::Terminated);
    rt.on_terminated(current_tid); // despierta joiners
    rt.clear_current()

}



#[derive(Debug)]
pub enum JoinError {
    NoSuchThread,
    NotJoinable,
    SelfJoin,
}

#[derive(Debug)]
pub enum DetachError {
    NoSuchThread,
    AlreadyDetached,
}

pub fn my_thread_join(
    rt: &mut ThreadRuntime,
    current: ThreadId,
    target: ThreadId,
) -> Result<(), JoinError> {
    if current == target {
        return Err(JoinError::SelfJoin);
    }
    let target_state = rt.get_state(target).ok_or(JoinError::NoSuchThread)?;
    let joinable = rt.is_joinable(target).ok_or(JoinError::NoSuchThread)?;
    if !joinable {
        return Err(JoinError::NotJoinable);
    }

    // Si ya terminó, retorno inmediato: no bloqueamos.
    if target_state == ThreadState::Terminated {
        return Ok(());
    }

    // Bloquea al current y registra dependencia
    rt.mark_blocked_on(current, target);
    rt.clear_current();
    Ok(())
}

pub fn my_thread_detach(rt: &mut ThreadRuntime, tid: ThreadId) -> Result<(), DetachError> {
    let joinable = rt.is_joinable(tid).ok_or(DetachError::NoSuchThread)?;
    if !joinable {
        return Err(DetachError::AlreadyDetached);
    }
    rt.set_joinable(tid, false);
    Ok(())
}


pub fn my_mutex_lock(rt: &mut ThreadRuntime, current: ThreadId, m: &mut MyMutex) {
    match m.owner {
        None => {
            m.owner = Some(current);
        }
        Some(_) => {
            m.waiters.push_back(current);
            rt.set_state(current, ThreadState::Blocked);
            rt.clear_current();
        }
    }
}

pub fn my_mutex_unlock(rt: &mut ThreadRuntime, current: ThreadId, m: &mut MyMutex) {
    if m.owner == Some(current) {
        if let Some(next) = m.waiters.pop_front() {
            // lo pasa a Ready y entrégale la tenencia
            rt.set_state(next, ThreadState::Ready);
            rt.enqueue(next);
            m.owner = Some(next);
        } else {
            m.owner = None;
        }
    } else {
        
    }
}
