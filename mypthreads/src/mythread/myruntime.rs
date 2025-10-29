use std::collections::{HashMap, VecDeque};
use std::os::raw::c_int;
use crate::mythread::mypthreadexits::Exits::{Ok, ThreadIsTerminated, UnknownThread};
use crate::mythread::mythread::{AnyParam, MyTRoutine, MyThread, ThreadId};
use crate::mythread::mythreadattr::MyThreadAttr;
use crate::mythread::thread_state::ThreadState;
use crate::Scheduler;
use crate::scheduler::SchedulerType;

use crate::scheduler::{round_robin::RRScheduler, lottery::LotteryScheduler, real_time::RealTimeScheduler};
pub struct MyTRuntime {
    pub(crate) time_ms: usize,
    pub(crate) run_queue: VecDeque<ThreadId>,
    pub(crate) threads: HashMap<ThreadId, MyThread>,
    pub(crate) next_id: ThreadId,
    pub(crate) current: Option<ThreadId>,
    pub(crate) wait_on: HashMap<ThreadId, Vec<ThreadId>>, // target -> waiters
    schedulers: HashMap<SchedulerType, Box<dyn Scheduler>>

}

impl MyTRuntime {
    pub fn new() -> Self {

        let mut schedulers: HashMap<SchedulerType, Box<dyn Scheduler>> = HashMap::new();
        schedulers.insert(SchedulerType::RoundRobin, Box::new(RRScheduler::new()));
        schedulers.insert(SchedulerType::Lottery,   Box::new(LotteryScheduler::new()));
        schedulers.insert(SchedulerType::RealTime,   Box::new(RealTimeScheduler::new()));

        Self {
            time_ms: 0,
            threads: HashMap::new(),
            run_queue: VecDeque::new(),
            next_id: 0,
            current: None,
            wait_on: HashMap::new(),
            schedulers,
        }
    }



        pub fn advance_steps(&mut self, passed: usize) {
        self.time_ms = self.time_ms.saturating_add(passed);
    }

    /// Crea un hilo en estado Ready y lo encola.
    pub fn create(&mut self,thread_out: *mut ThreadId,attr: *mut MyThreadAttr,start_routine: MyTRoutine,args: *mut AnyParam,scheduler: Option<SchedulerType>,
    ) -> c_int {
        let id = self.next_id;
        self.next_id += 1;

        let sched = scheduler.unwrap_or_default();
        let mut new_thread = MyThread::new(id, attr, start_routine, args, Default::default());
        new_thread.state = ThreadState::Ready;


        self.threads.insert(id, new_thread);


        if let Some(s) = self.schedulers.get_mut(&sched) {
            let t = self.threads.get(&id).unwrap();
            s.enqueue(id, t);
        } else {
            return -1; // no debería ocurrir
        }


        if !thread_out.is_null() {
            unsafe { *thread_out = id; }
        }

        0 // Exito en C
    }




    /// Política global: RealTime > Lottery > RoundRobin
    fn pick_any_next(&mut self) -> Option<ThreadId> {
        for kind in [SchedulerType::RealTime, SchedulerType::Lottery, SchedulerType::RoundRobin] {
            if let Some(s) = self.schedulers.get_mut(&kind) {
                if !s.is_empty() {
                    if let Some(tid) = s.pick_next() {
                        return Some(tid);
                    }
                }
            }
        }
        None
    }


    fn run_thread(&mut self, tid: ThreadId) {
        // Aquí haces el “contexto”/ejecución. En esta base: run-to-completion.
        let (routine, arg, scheduler_kind, detached);
        {
            let t = self.threads.get_mut(&tid).unwrap();
            t.state = ThreadState::Running;
            routine = t.start_routine;
            arg = t.arg;
            scheduler_kind = t.scheduler;
            detached = unsafe { (*t.attr).detached };
        }

        let ret = (routine)(arg);

        {
            let t = self.threads.get_mut(&tid).unwrap();
            t.ret_val = ret;
            t.state = ThreadState::Terminated;
        }

        if let Some(s) = self.schedulers.get_mut(&scheduler_kind) {
            s.on_exit(tid);
        }

        // Despertar joiners, cleanup si detached, etc.
        self.wake_joiners(&tid);
        if detached {
            // reclaim resources si lo queremos implementar
        }
    }

    /// Ejecuta un próximo hilo si existe (scheduler decide).
    pub fn schedule_next(&mut self) -> c_int {
        if let Some(next) = self.pick_any_next() {
            self.current = Some(next);

            if let Some(th) = self.threads.get(&next) {
                if th.state == ThreadState::Terminated {
                    // Nada que hacer; despierta joiners y sigue
                    self.wake_joiners(&next);
                    return 0;
                }
            }

            self.run_thread(next);
            0
        } else {
            self.current = None;
            1 // nada para correr
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

    /*
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
*/


    /// join controlado por el scheduler.
    /// Bloquea el hilo actual hasta que `tid` termine.
    /// ya no tiene run
    pub fn join(&mut self, target: ThreadId, ret_val_out: *mut *mut AnyParam) -> c_int {
        //  Validaciones básicas

        // Asegurar que el target exista
        let target_exists = match self.threads.get(&target) {
            Some(t) => t, None => return -1, // Hilo objetivo no existe
        };

        // No join sobre detached
        let is_detached = unsafe { (*target_exists.attr).detached };
        if is_detached {return -1;}

        // Si ya terminó, retorna inmediatamente su valor
        if target_exists.state == ThreadState::Terminated {
            if !ret_val_out.is_null() {unsafe { *ret_val_out = target_exists.ret_val as *mut AnyParam; }}
            return 0;
        }

        // MODO DRIVER: no hay hilo actual (join desde el hilo de prueba / fuera del runtime)
        if self.current.is_none() {
            loop {
                // ¿terminó el target?
                let done = match self.threads.get(&target) { Some(t) => t.state == ThreadState::Terminated, None => true };
                if done {break;}
                // Avanzar el scheduler, si no hay nada para correr y no terminó -> deadlock
                if self.schedule_next() != 0 {return -1;}
            }
            if let Some(t) = self.threads.get(&target) {
                if !ret_val_out.is_null() {unsafe { *ret_val_out = t.ret_val as *mut AnyParam; }}
            } else {
                if !ret_val_out.is_null() {unsafe { *ret_val_out = std::ptr::null_mut(); }}
            }
            return 0;
        }

        // MODO RUNTIME: hay hilo actual, aplicar bloqueo y espera
        let current_tid = match self.current {Some(id) => id, None => unreachable!()};
        if current_tid == target {return -1;}

        // Registrar que el hilo "current" espera a "target" (un solo joiner)
        {
            let waiters = self.wait_on.entry(target).or_default();
            if waiters.iter().any(|&w| w == current_tid) {return -1;}
            if !waiters.is_empty() {return -1;} waiters.push(current_tid);
        }

        // Bloquear al hilo actual y ceder el CPU
        {
            let cur = self.threads.get_mut(&current_tid).unwrap();
            cur.state = ThreadState::Blocked;
        }

        // dejamos que el scheduler corra otros hilos hasta que el objetivo termine (wake_joiners lo reactivará) y
        // nos vuelvan a programar (self.current == Some(current_tid))
        loop {
            // Si el objetivo ya terminó, salimos del loop para devolver ret_val
            if let Some(t) = self.threads.get(&target) {if t.state == ThreadState::Terminated {break;}}
            else {break;}

            // Pedimos al runtime que ejecute el siguiente hilo disponible schedule_next() hará run-to-completion del elegido,
            // y al terminar "target", llamará a wake_joiners() que nos re-encola.
            if self.schedule_next() != 0 {return -1;}

            // Si ya volvimos a ser el hilo actual, revisa nuevamente el estado del target.
            if self.current == Some(current_tid) {
                if let Some(t) = self.threads.get(&target) { if t.state == ThreadState::Terminated {break;} }
                else {break;}
            }
        }

        // El target está Terminated Recupera el ret_val y limpia la espera.
        if let Some(t) = self.threads.get(&target) {
            if !ret_val_out.is_null() {unsafe { *ret_val_out = t.ret_val as *mut AnyParam; }}
        } else {
            if !ret_val_out.is_null() {unsafe { *ret_val_out = std::ptr::null_mut(); }}
        }

        // Limpiar la lista de waiters para este target
        self.wait_on.remove(&target);

        0
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
