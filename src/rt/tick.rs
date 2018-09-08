use ::cortex_m::interrupt;

static mut TICK_COUNT: u64 = 0;

pub fn inc() {
    interrupt::free(|_| unsafe {
        TICK_COUNT = TICK_COUNT.overflowing_add(1).0;
    });
}

pub fn count() -> u64 {
    unsafe {
        TICK_COUNT
    }
}
