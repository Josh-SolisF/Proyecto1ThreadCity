pub mod round_robin;
pub mod lottery;
pub mod scheduler_type;
pub mod scheduler_params;
pub(crate) mod real_time;


pub use real_time::RealTimeScheduler;
pub use scheduler_type::SchedulerType;
pub use scheduler_params::SchedulerParams;

use crate::{mythread::mythread::MyThread};
use crate::mythread::mythread::ThreadId;


pub trait Scheduler {
    // Encola un hilo listo
    fn enqueue(&mut self, tid: ThreadId, t: &MyThread);

    // Saca el siguiente hilo a ejecutar (None si vacío)
    fn pick_next(&mut self) -> Option<ThreadId>;

    // Eventos
    fn on_block(&mut self, _tid: ThreadId) {}
    fn on_exit(&mut self, _tid: ThreadId) {}

    fn is_empty(&self) -> bool;
}

