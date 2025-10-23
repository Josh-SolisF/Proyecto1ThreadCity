#[repr(C)]
#[derive(Clone, Copy)]
pub struct MyThreadAttr {
    pub scheduler: u8,     // 0=RR, 1=Lottery, 2=RealTime
    pub priority: u8,
    pub stack_size: usize,
    pub detached: bool,
}

impl Default for MyThreadAttr {
    fn default() -> Self {
        Self {
            scheduler: 0,
            priority: 0,
            stack_size: 0,
            detached: false,
        }
    }
}