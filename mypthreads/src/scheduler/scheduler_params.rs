pub enum SchedulerParams {
    None,
    Lottery, //(tickets) u 32
    RealTime, //(deadline_ms) u64
    Priority // i32
}