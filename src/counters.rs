#[derive(Debug)]
pub struct Counters {
    ///Number of interrupts/exceptions
    pub interrupt: u64,
}

impl Counters {
    pub const fn new() -> Self {
        Self {
            interrupt: 0
        }
    }
}
