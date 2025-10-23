
#[repr(u8)]
pub enum SchedulerType {
    RoundRobin,
    Lottery,
    RealTime,
}
// To match with C scheduling values, idk if they have this order tho.
pub fn scheduler_value(my_sched: SchedulerType) -> u8 {
    my_sched as u8
}