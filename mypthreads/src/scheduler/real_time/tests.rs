// scheduler/real_time.rs (o en tests/real_time_tests.rs)
#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;
    use crate::scheduler::scheduler_type::SchedulerType;
    use crate::mythread::mythread::{MyThread, ThreadId, AnyParam};
    use crate::mythread::mythreadattr::MyThreadAttr;
    use crate::Scheduler;
    use crate::scheduler::RealTimeScheduler;

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
    fn edf_picks_earliest_deadline_first() {
        let mut edf = RealTimeScheduler::new();

        let (t1, _a1) = make_thread(1, 0, 50, SchedulerType::RealTime);
        let (t2, _a2) = make_thread(2, 0, 10, SchedulerType::RealTime);
        let (t3, _a3) = make_thread(3, 0, 30, SchedulerType::RealTime);

        edf.enqueue(1, &t1);
        edf.enqueue(2, &t2);
        edf.enqueue(3, &t3);

        assert_eq!(edf.pick_next(), Some(2)); // 10
        assert_eq!(edf.pick_next(), Some(3)); // 30
        assert_eq!(edf.pick_next(), Some(1)); // 50
        assert_eq!(edf.pick_next(), None);
        assert!(edf.is_empty());
    }

    #[test]
    fn edf_tie_breaker_by_tid_when_same_deadline() {
        let mut edf = RealTimeScheduler::new();

        let (t1, _a1) = make_thread(10, 0, 100, SchedulerType::RealTime);
        let (t2, _a2) = make_thread(5,  0, 100, SchedulerType::RealTime); // mismo deadline, menor tid

        edf.enqueue(10, &t1);
        edf.enqueue(5,  &t2);

        // con Reverse((deadline, tid)), gana el tid más bajo en empate
        assert_eq!(edf.pick_next(), Some(5));
        assert_eq!(edf.pick_next(), Some(10));
        assert_eq!(edf.pick_next(), None);
    }

    #[test]
    fn edf_empty_returns_none() {
        let mut edf = RealTimeScheduler::new();
        assert!(edf.is_empty());
        assert_eq!(edf.pick_next(), None);
    }
}
