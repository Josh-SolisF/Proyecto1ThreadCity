// scheduler/lottery.rs (o en tests/lottery_tests.rs)
#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;
    use crate::scheduler::scheduler_type::SchedulerType;
    use crate::mythread::mythread::{MyThread, ThreadId, AnyParam};
    use crate::mythread::mythreadattr::MyThreadAttr;
    use crate::Scheduler;
    use crate::scheduler::lottery::LotteryScheduler;

    extern "C" fn dummy(_arg: *mut AnyParam) -> *mut c_void {
        std::ptr::null_mut()
    }

    fn make_thread(
        id: ThreadId,
        tickets: u8,
        deadline: usize,
        sched: SchedulerType,
    ) -> (MyThread, Box<MyThreadAttr>) {
        let mut attr = Box::new(MyThreadAttr::new(deadline, tickets));
        let t = MyThread::new(id, &mut *attr, dummy, std::ptr::null_mut(), Some(sched));
        (t, attr)
    }

    #[test]
    fn lottery_bias_matches_tickets_ratio() {
        let mut lot = LotteryScheduler::new();

        // t1: 1 ticket, t2: 9 tickets => ~10% vs ~90%
        let (t1, _a1) = make_thread(1, 1, usize::MAX, SchedulerType::Lottery);
        let (t2, _a2) = make_thread(2, 9, usize::MAX, SchedulerType::Lottery);

        let trials = 20_000;
        let mut c1 = 0usize;
        let mut c2 = 0usize;

        for _ in 0..trials {
            // En cada muestra, encola ambos y extrae una sola selección.
            lot.enqueue(1, &t1);
            lot.enqueue(2, &t2);
            let draw = lot.pick_next().unwrap();
            // Saca el otro para limpiar el estado (no lo contamos)
            let _ = lot.pick_next();

            match draw {
                1 => c1 += 1,
                2 => c2 += 1,
                _ => unreachable!(),
            }
        }

        let p1 = c1 as f64 / trials as f64;
        let p2 = c2 as f64 / trials as f64;

        // 1/10 y 9/10 con tolerancia
        assert!(p1 > 0.07 && p1 < 0.13, "p1={p1}, counts=({c1},{c2})");
        assert!(p2 > 0.87 && p2 < 0.93, "p2={p2}, counts=({c1},{c2})");
    }

    #[test]
    fn lottery_min_one_ticket_even_if_zero() {
        let mut lot = LotteryScheduler::new();

        // priority=0 -> tickets=1 por max(1)
        let (t1, _a1) = make_thread(1, 0, usize::MAX, SchedulerType::Lottery);
        let (t2, _a2) = make_thread(2, 0, usize::MAX, SchedulerType::Lottery);

        let trials = 6_000;
        let mut c1 = 0usize;
        let mut c2 = 0usize;

        for _ in 0..trials {
            lot.enqueue(1, &t1);
            lot.enqueue(2, &t2);
            let draw = lot.pick_next().unwrap();
            let _ = lot.pick_next(); // limpiar
            match draw {
                1 => c1 += 1,
                2 => c2 += 1,
                _ => unreachable!(),
            }
        }

        // Con ambos = 1 ticket, prob ~50/50 con tolerancia
        let p1 = c1 as f64 / trials as f64;
        assert!(p1 > 0.45 && p1 < 0.55, "p1={p1}, c1={c1}, c2={c2}");
    }

    #[test]
    fn lottery_empty_returns_none() {
        let mut lot = LotteryScheduler::new();
        assert!(lot.is_empty());
        assert_eq!(lot.pick_next(), None);
    }
}