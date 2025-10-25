use std::collections::{HashMap, VecDeque};
use std::os::raw::c_int;
use crate::mythread::mythread::{AnyParam, MyTRoutine, MyThread, ThreadId};
use crate::mythread::mythreadattr::{MyAttr, MyThreadAttr};
use crate::mythread::thread_state::ThreadState;

pub struct MyTRuntime {
    pub(crate) run_queue: VecDeque<ThreadId>,
    pub(crate) threads: HashMap<ThreadId, MyThread>,
    next_id: ThreadId,
    pub(crate) current: Option<ThreadId>,
    wait_on: HashMap<ThreadId, Vec<ThreadId>>, // target -> waiters

}

impl MyTRuntime {
    pub fn new() -> Self {
        Self {
            threads: HashMap::new(),
            run_queue: VecDeque::new(),
            next_id: 1,
            current: None,
            wait_on: HashMap::new(),
        }
    }

    /// Crea un hilo en estado Ready y lo encola.
    pub fn create(&mut self,
                  thread_out: *mut ThreadId,
                  attr: *const MyAttr,
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
    pub fn schedule_next(&mut self) {
        if let Some(next) = self.run_queue.pop_front() {
            self.current = Some(next);
            if let Some(th) = self.threads.get_mut(&next) {
                if th.state != ThreadState::Terminated {
                    th.state = ThreadState::Running;
                }
            }
        } else {
            // No hay hilos listos; queda sin current
            self.current = None;
        }
    }


    /// Cambia el estado del hilo si existe.
    pub fn set_state(&mut self, tid: ThreadId, st: ThreadState) {
        if let Some(t) = self.threads.get_mut(&tid) {
            t.state = st;
        }
    }


    /// Marca al hilo actual como Ready, lo reencola y limpia current.
    /// Luego intenta seleccionar el siguiente (no ejecuta la rutina aquí).
    pub fn yield_current(&mut self) -> c_int {
        if let Some(cur) = self.current {
            // Si existe el hilo en el mapa, cambia su estado a Ready
            if let Some(th) = self.threads.get_mut(&cur) {
                // Sólo si no está terminado, lo ponemos Ready
                if th.state != ThreadState::Terminated {
                    th.state = ThreadState::Ready;
                    self.enqueue(cur);
                }
            }
            self.clear_current();
        }
        // Intenta seleccionar el siguiente a correr
        self.schedule_next();
        0
    }

    /// Variante que recibe un TID explícito. Si coincide con el current, hace yield.
    /// Si no coincide, intenta mover ese TID a Ready (si existe) y reencolarlo.
    pub fn yield_thread(&mut self, tid: ThreadId) -> c_int {
        match self.current {
            Some(cur) if cur == tid => self.yield_current(),
            _ => {
                if let Some(th) = self.threads.get_mut(&tid) {
                    if th.state != ThreadState::Terminated {
                        th.state = ThreadState::Ready;
                        self.enqueue(tid);
                    }
                    0
                } else {
                    -1 // No existe el hilo
                }
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


    /// Ejecuta un “paso” cooperativo: elige, marca Running y fija `current`.
   /* /// El código de hilo (o tests) deberían llamar luego a `yield/end`.
    pub fn step(&mut self) -> Option<ThreadId> {
        let next = self.schedule_next()?;
        self.set_state(next, ThreadState::Running);
        self.current = Some(next);
        Some(next)
    }
*/
    /// Útil cuando el hilo hace yield/end: limpiamos `current`.

    pub fn clear_current(&mut self) {
        self.current = None;
    }


    pub fn end_current(&mut self, retval: *mut AnyParam) -> c_int {
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
        if let Some(waiters) = self.wait_on.remove(&cur) {
            for w in waiters {
                if let Some(tw) = self.threads.get_mut(&w) {
                    if tw.state == ThreadState::Blocked {
                        tw.state = ThreadState::Ready;
                        self.run_queue.push_back(w);
                    }
                }
            }
        }

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

}
