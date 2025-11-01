#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrafficLight {
    pub(crate) in_red: bool,
    pub(crate) time_passed_ms: usize,
    pub(crate) update_interval_ms: usize,
}

impl TrafficLight {
    pub fn new(update_interval_ms: usize) -> TrafficLight {
        Self {
            in_red: false,
            time_passed_ms: 0,
            update_interval_ms
        }
    }

    pub fn can_pass(&self) -> bool {
        !self.in_red
    }
    pub fn advance_time(&mut self, time_passed: usize) {
        self.time_passed_ms += time_passed;
        if self.time_passed_ms >= self.update_interval_ms {
            self.time_passed_ms -= self.update_interval_ms;
            self.in_red = !self.in_red;
        }
    }
}