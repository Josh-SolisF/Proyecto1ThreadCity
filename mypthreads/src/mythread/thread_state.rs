#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    New,
    Ready,
    Running,
    Blocked,
    Terminated,
}




