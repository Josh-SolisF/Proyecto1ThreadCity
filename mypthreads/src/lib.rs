pub mod mythread;
pub mod scheduler;

#[cfg(test)]

mod tests;



pub use scheduler::Scheduler;
pub use scheduler::scheduler_type::SchedulerType;
pub use scheduler::round_robin::RRScheduler as RoundRobinScheduler;
pub use scheduler::lottery::LotteryScheduler;
pub use scheduler::real_time::RealTimeScheduler;
