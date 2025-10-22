mod tests;

use crate::Scheduler;

pub struct RoundRobinScheduler;
impl Scheduler for RoundRobinScheduler {}