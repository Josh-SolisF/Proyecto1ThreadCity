use std::sync::Mutex;
use crate::mythread::thread_state::ThreadState;
use crate::scheduler::scheduler_type::SchedulerType;
use crate::scheduler::scheduler_params::SchedulerParams;
pub type ThreadId = u32;
pub struct MyThread {
    pub id: ThreadId,
    //pub tid: Option<ThreadId>,
    pub state: Mutex<ThreadState>,
    pub sched_type: Mutex<SchedulerType>,
    pub sched_data: Mutex<SchedulerParams>,
    // Usamos Option para poder 'take()' el closure temporalmente sin doble prestamo.
}

impl MyThread {
    pub fn new(id: ThreadId, sched: SchedulerType, params: SchedulerParams) -> Self {
        Self {
            id,
          //  tid,
            state: Mutex::new(ThreadState::Ready),
            sched_type: Mutex::new(sched),
            sched_data: Mutex::new(params),
        }
    }

}



