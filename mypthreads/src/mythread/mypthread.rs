use std::os::raw::c_int;
use std::cell::UnsafeCell;
use crate::mythread::mymutex::MyMutex;
use crate::mythread::myruntime::MyTRuntime;
use crate::mythread::mythread::{AnyParam, MyTRoutine, ThreadId};
use crate::mythread::mythreadattr::{MyAttr, MyThreadAttr};
use crate::mythread::thread_state::ThreadState;

pub struct MyGlobalRuntime {
    inner: UnsafeCell<Option<MyTRuntime>>,
}
unsafe impl Sync for MyGlobalRuntime {
}

/*
impl MyGlobalRuntime {
    pub const fn new() -> Self {
        Self { inner: UnsafeCell::new(None) }
    }
    pub fn get_mut(&self) -> &mut Option<MyTRuntime> {
        // encapsulamos el unsafe aquí
        unsafe { &mut *self.inner.get() }
    }
}
*/


impl MyGlobalRuntime {
    pub const fn new() -> Self { Self { inner: UnsafeCell::new(None) } }
    pub fn get_mut(&self) -> &mut Option<MyTRuntime> { unsafe { &mut *self.inner.get() } }
}


static RUNTIME: MyGlobalRuntime = MyGlobalRuntime::new();

fn ensure_runtime<'a>() -> &'a mut MyTRuntime {
    let r = RUNTIME.get_mut();
    if r.is_none() { *r = Some(MyTRuntime::new()); }
    r.as_mut().unwrap()
}

/// Crea un hilo del mismo modo que lo haría la biblioteca pthread
///
/// # Arguments
/// * `thread` - Es donde se almacenará el id del nuevo hilo.
/// * `_attr` - Se espera un MyThreadAttr que usara el hilo para configurarse.
/// * `start_routine` - Es la rutina/función a ejecutar por el hilo.
/// * `arg` - Son los parametros que requiera `start_routine` para ejecutarse.
///
/// # Returns
/// `0` de c_int

#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_thread_create(
    thread: *mut ThreadId,
    attr: *const MyAttr,
    start_routine: MyTRoutine,
    arg: *mut AnyParam,
) -> c_int {

    let rt = ensure_runtime();

    let attr_ref: *const MyAttr = if attr.is_null() {
        &MyAttr::default()
    } else {
        attr
    };

    rt.create(thread, attr_ref, start_routine, arg)
}




#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_thread_join(
    thread: ThreadId,
    ret_val: *mut *mut AnyParam,
) -> c_int {

    let rt = ensure_runtime();
    rt.join(thread, ret_val)

}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_thread_yield(tid: ThreadId) -> c_int {

    let rt = ensure_runtime();
    rt.yield_current()

}

#[unsafe(no_mangle)]

pub unsafe extern "C" fn my_thread_end(retval: *mut AnyParam) -> c_int {
    let rt = ensure_runtime();
    rt.end_current(retval)
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_thread_run_scheduler() -> c_int {
    let rt = ensure_runtime();
    rt.run_until_idle();
    0
}


/*
  etorno inmediato: no bloqueamos.
    if target_state == ThreadState::Terminated {
        return Ok(());
    }

    // Bloquea al current y registra dependencia
    rt.mark_blocked_on(current, target);
    rt.clear_current();
    Ok(())
}

/// El hilo actual cede la CPU .
/// Convención: quien llama pasa su propio `current_tid`.
pub fn my_thread_yield(rt: &mut MyTRuntime, current_tid: ThreadId) {
    // Marcar Ready y reencolar. (Suponemos estaba Running.)
    rt.set_state(current_tid, ThreadState::Ready);
    rt.enqueue(current_tid);
    rt.clear_current();
    // aqui iria `schedule_next()` y “cambiará” de hilo.
}

/// El hilo actual termina su ejecución.
pub fn my_thread_end(rt: &mut MyTRuntime, current_tid: ThreadId) {
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


pub fn my_thread_detach(rt: &mut MyTRuntime, tid: ThreadId) -> Result<(), DetachError> {
    let joinable = rt.is_joinable(tid).ok_or(DetachError::NoSuchThread)?;
    if !joinable {
        return Err(DetachError::AlreadyDetached);
    }
    rt.set_joinable(tid, false);
    Ok(())
}


pub fn my_mutex_lock(rt: &mut MyTRuntime, current: ThreadId, m: &mut MyMutex) {
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

pub fn my_mutex_unlock(rt: &mut MyTRuntime, current: ThreadId, m: &mut MyMutex) {
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

*/