use std::os::raw::c_int;
use std::cell::UnsafeCell;
use crate::mythread::mymutex::MyMutex;
use crate::mythread::myruntime::MyTRuntime;
use crate::mythread::mythread::{AnyParam, MyTRoutine, ThreadId};
use crate::mythread::mythreadattr::{MyAttr, MyThreadAttr};
use crate::mythread::thread_state::ThreadState;
use crate::Scheduler;
use crate::scheduler::r#trait::DefaultScheduler;

struct MyGlobalRuntime {
    inner: UnsafeCell<Option<MyTRuntime<MyScheduler>>>,
}
unsafe impl Sync for MyGlobalRuntime {
}


impl MyGlobalRuntime {
    pub const fn new() -> Self {
        Self { inner: UnsafeCell::new(None) }
    }
    pub fn get_mut(&self) -> &mut MyTRuntime {
        let inner = unsafe { &mut *self.inner.get() };
        if inner.is_none() {*inner = Some(MyTRuntime::new())}
        inner.as_mut().unwrap()
    }
}


static RUNTIME: MyGlobalRuntime = MyGlobalRuntime::new();

/// Crea un hilo del mismo modo que lo haría la biblioteca pthread
///
/// # Arguments
/// * `thread` - Es donde se almacenará el ID del nuevo hilo.
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
    let runtime = RUNTIME.get_mut();
    let attr_ref: *const MyAttr = if attr.is_null() {
        let default_attr = MyThreadAttr::new();
        default_attr.c_pointer()
    } else {
        attr
    };

    runtime.create(thread, attr_ref, start_routine, arg)
}




#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_thread_join(
    thread: ThreadId,
    ret_val: *mut *mut AnyParam,
) -> c_int {
    let runtime = RUNTIME.get_mut();
    runtime.join(thread, ret_val)
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_thread_yield() -> c_int {
    let rt = RUNTIME.get_mut();
    unsafe {
        rt.save_context();
    }
    0

}

#[unsafe(no_mangle)]

pub unsafe extern "C" fn my_thread_end(retval: *mut AnyParam) -> c_int {
    let runtime = RUNTIME.get_mut();
    runtime.end_current(retval)
}


/*
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