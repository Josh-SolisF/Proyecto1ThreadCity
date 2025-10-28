use std::collections::{HashMap, VecDeque};
use std::os::raw::c_int;
use crate::mythread::mypthreadexits::Exits::{Ok, ThreadIsTerminated, UnknownThread};
use crate::mythread::mythread::{AnyParam, MyTRoutine, MyThread, ThreadId};
use crate::mythread::mythreadattr::MyThreadAttr;
use crate::mythread::thread_state::ThreadState;
use crate::Scheduler;

pub struct MyTRuntime {
    pub(crate) time_ms: usize,
    pub(crate) run_queue: VecDeque<ThreadId>,
    pub(crate) threads: HashMap<ThreadId, MyThread>,
    pub(crate) next_id: ThreadId,
    pub(crate) current: Option<ThreadId>,
    pub(crate) wait_on: HashMap<ThreadId, Vec<ThreadId>>, // target -> waiters
    pub(crate) my_schedulers: Vec<Box<dyn Scheduler>>,
}

impl MyTRuntime {
    pub fn new(schedulers: Vec<Box<dyn Scheduler>>) -> Self {
        Self {
            time_ms: 0,
            threads: HashMap::new(),
            run_queue: VecDeque::new(),
            next_id: 0,
            current: None,
            wait_on: HashMap::new(),
            my_schedulers: schedulers
        }
    }
    
    pub fn advance_steps(&mut self, passed: usize) {
        self.time_ms = self.time_ms.saturating_add(passed);
    }

    /// Crea un hilo en estado Ready y lo encola.
    pub fn create(&mut self,
                  thread_out: *mut ThreadId,
                  attr: *mut MyThreadAttr,
                  start_routine: MyTRoutine,
                  args: *mut AnyParam,
    ) -> c_int {
        let id = self.next_id;
        self.next_id += 1;


        let mut new_thread = MyThread::new(id, attr, start_routine, args);
        new_thread.state = ThreadState::Ready;


        self.threads.insert(id, new_thread);
        self.run_queue.push_back(id);

        if !thread_out.is_null() {
            unsafe { *thread_out = id; }
        }

        0 // Exito en C
    }

    /// Selecciona el siguiente hilo en la cola (Round Robin básico).

    /// Selecciona el siguiente hilo listo y lo marca como Running.
    /// (No invoca la rutina aquí; el baseline ejecuta en join()).
    pub fn schedule_next(&mut self) -> c_int {
        let candidates = self.my_schedulers.iter().map(|sch|
            sch.schedule(&mut self.run_queue, &mut self.threads)).collect::<Vec<_>>();

        // TODO: Elegir cual candiato es el más apto, de momento solo tomaré el primero del queue y ya
        if let Some(next) = self.run_queue.pop_front() {
            self.current = Some(next);
            if let Some(th) = self.threads.get_mut(&next) {
                if th.state != ThreadState::Terminated {
                    th.state = ThreadState::Running;
                    th.run();
                }
            }

            if let Some(th) = self.threads.get(&next) {
                if th.state == ThreadState::Terminated {
                    self.wake_joiners(&next);
                }
            }
            0

        } else {
            self.current = None;
            1
        }
    }

    /// Cambia el estado del hilo si existe.
    pub fn set_state(&mut self, tid: ThreadId, st: ThreadState) {
        if let Some(t) = self.threads.get_mut(&tid) {
            t.state = st;
        }
    }

    pub fn save_context(&mut self) {
        if let Some(tid) = self.current {
            if let Some(th) = self.threads.get_mut(&tid) {
                th.state = ThreadState::Ready;
            }
        }
    }

    /// Devuelve el estado actual (útil para tests).
    pub fn get_state(&self, tid: ThreadId) -> Option<ThreadState> {
        self.threads.get(&tid).map(|t| t.state)
    }

 /*   /// Reencola un hilo.
    pub fn enqueue(&mut self, tid: ThreadId) {
        // Solo encolamos si no está Terminated.
        if matches!(self.get_state(tid), Some(ThreadState::Ready | ThreadState::Running | ThreadState::Blocked)) {
            self.run_queue.push_back(tid);
        }
    }
*/

    pub fn enqueue(&mut self, tid: ThreadId) {
        self.run_queue.push_back(tid);
    }


    /// Útil cuando se hace end: limpiamos `current`.

    pub fn clear_current(&mut self) {
        self.current = None;
    }

    pub unsafe fn detach(&mut self, tid: ThreadId) -> c_int {
        if let Some(th) = self.threads.get_mut(&tid) {
            unsafe { th.attr.as_mut().unwrap().detach(); }
            if th.state == ThreadState::Terminated {
                self.threads.remove(&tid);
            }
            0
        } else {
            libc::ESRCH
        }
    }

    pub fn end_current(&mut self, retval: *mut AnyParam, ) -> c_int {
        let Some(cur) = self.current else {
            // No hay hilo en ejecución; no debería pasar, pero devolvemos error
            return -1;
        };

        // Marca terminado y guarda el retorno
        if let Some(th) = self.threads.get_mut(&cur) {
            th.ret_val = retval;
            th.state = ThreadState::Terminated;
        } else {
            return -1; // el TID actual no está en el mapa (inconsistencia)
        }

        // Despierta a los joiners (si implementaste join bloqueante)
        self.wake_joiners(&cur);

        // Limpia current y selecciona siguiente
        self.clear_current();
        self.schedule_next();
        0
    }


    pub fn get_current(&self) -> Option<ThreadId> {
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

    pub fn join(&mut self, tid: ThreadId, ret_val: *mut *mut AnyParam) -> c_int {
        if let Some(th) = self.threads.get_mut(&tid) {
            // Ejecuta una sola vez
            if th.state != ThreadState::Terminated {
                th.run();
            }
            if !ret_val.is_null() {
                unsafe { *ret_val = th.ret_val }
            }
            return 0;
        }
        -1 // No existe el thread
    }

    pub fn wake_thread(&mut self, target: ThreadId) -> c_int {
        if let Some(th) = self.threads.get_mut(&target) {
            if th.state != ThreadState::Terminated {
                th.state = ThreadState::Ready;
                return Ok as c_int;
            }
            return ThreadIsTerminated as c_int;
        }
        UnknownThread as c_int
    }
    
    fn wake_joiners(&mut self, objective: &ThreadId) {
        if let Some(waiters) = self.wait_on.remove(objective) {
            for w in waiters {
                if let Some(tw) = self.threads.get_mut(&w) {
                    if tw.state == ThreadState::Blocked {
                        tw.state = ThreadState::Ready;
                        self.run_queue.push_back(w);
                    }
                }
            }
        }
    }
}
