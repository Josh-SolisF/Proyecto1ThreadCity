// scheduler/round_robin.rs (o en tests/round_robin_tests.rs)
#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;
    use crate::scheduler::scheduler_type::SchedulerType;
    use crate::mythread::mythread::{MyThread, ThreadId, AnyParam};
    use crate::mythread::mythreadattr::MyThreadAttr;
    use crate::mythread::thread_state::ThreadState;
    use crate::Scheduler;
    use crate::scheduler::round_robin::RRScheduler;

    extern "C" fn dummy(_arg: *mut AnyParam) -> *mut c_void {
        std::ptr::null_mut()
    }

    fn make_thread(
        id: ThreadId,
        priority: u8,
        deadline: usize,
        sched: SchedulerType,
    ) -> (MyThread, Box<MyThreadAttr>) {
        let mut attr = Box::new(MyThreadAttr::new(deadline, priority));
        let t = MyThread::new(id, &mut *attr, dummy, std::ptr::null_mut(), Some(sched));
        (t, attr)
    }

    #[test]
    fn rr_basic_fifo() {
        let mut rr = RRScheduler::new();

        let (t1, _a1) = make_thread(1, 10, usize::MAX, SchedulerType::RoundRobin);
        let (t2, _a2) = make_thread(2, 20, usize::MAX, SchedulerType::RoundRobin);
        let (t3, _a3) = make_thread(3, 30, usize::MAX, SchedulerType::RoundRobin);

        rr.enqueue(1, &t1);
        rr.enqueue(2, &t2);
        rr.enqueue(3, &t3);

        assert_eq!(rr.pick_next(), Some(1));
        assert_eq!(rr.pick_next(), Some(2));
        assert_eq!(rr.pick_next(), Some(3));
        assert_eq!(rr.pick_next(), None);
        assert!(rr.is_empty());
    }

    #[test]
    fn rr_cycle_with_reenqueue() {
        let mut rr = RRScheduler::new();
        let (t1, _a1) = make_thread(1, 10, usize::MAX, SchedulerType::RoundRobin);
        let (t2, _a2) = make_thread(2, 10, usize::MAX, SchedulerType::RoundRobin);

        rr.enqueue(1, &t1);
        rr.enqueue(2, &t2);

        // pick 1 -> re-encolar
        assert_eq!(rr.pick_next(), Some(1));
        rr.enqueue(1, &t1);

        // pick 2 -> re-encolar
        assert_eq!(rr.pick_next(), Some(2));
        rr.enqueue(2, &t2);

        // pick 1 otra vez
        assert_eq!(rr.pick_next(), Some(1));
    }

    #[test]
    fn rr_empty_returns_none() {
        let mut rr = RRScheduler::new();
        assert!(rr.is_empty());
        assert_eq!(rr.pick_next(), None);
    }
}