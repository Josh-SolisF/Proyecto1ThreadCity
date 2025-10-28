
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub enum SchedulerType {
    RoundRobin,
    Lottery,
    RealTime,
}


impl Default for SchedulerType {
    fn default() -> Self { SchedulerType::RoundRobin }
}
