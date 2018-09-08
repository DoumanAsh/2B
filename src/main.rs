#![no_main]
#![no_std]

extern crate stm32l4x6_hal as hal;
extern crate cortex_m;
extern crate cortex_m_rt;
#[cfg(feature = "debug")]
extern crate panic_semihosting;
#[cfg(feature = "release")]
extern crate panic_abort;
#[cfg(feature = "debug")]
extern crate cortex_m_log;
extern crate embedded_hal;

use embedded_hal::digital::{ToggleableOutputPin};
use cortex_m::asm::wfe;
use cortex_m_rt::{entry, exception};

use core::hint;

#[macro_use]
mod rt;

static mut RT: Option<rt::Guard> = None;

#[inline]
fn get_rt() -> &'static mut rt::Guard {
    unsafe {
        match RT.as_mut() {
            Some(rt) => rt,
            None => hint::unreachable_unchecked(),
        }
    }
}

#[inline]
fn init() {
    unsafe {
        RT = Some(rt::init())
    }
}

#[entry]
fn main() -> ! {
    init();

    log!("Initialize firmware");
    // infinite loop; just so we don't leave this stack frame
    loop {
        wfe();
    }
}

#[exception]
fn DefaultHandler(irqn: i16) {
    log!("DefaultHandler: IRQn = {}", irqn);
}

#[exception]
fn SysTick() {
    rt::tick::inc();

    let rt = get_rt();

    rt.device.led.red.toggle();
}
