#![no_main]
#![no_std]

extern crate stm32l4x6_hal as hal;
extern crate cortex_m;
extern crate cortex_m_rt;
#[cfg(debug_assertions)]
extern crate cortex_m_log;
extern crate embedded_hal;
extern crate nb;

use embedded_hal::serial::{Write};
use embedded_hal::digital::{ToggleableOutputPin};
use cortex_m::asm::wfe;
use cortex_m_rt::{entry, exception};

use core::hint;

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
    const WELCOME: &'static [u8; 12] = b"Hello world!";
    unsafe {
        let mut rt = rt::init();

        for byte in WELCOME {
            match nb::block!(rt.device.serial.write(*byte)) {
                Ok(_) => (),
                Err(error) => log!("Error while writing welcome: {:?}", error),
            }
        }

        RT = Some(rt);
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

#[allow(unused)]
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

#[exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    // prints the exception frame as a panic message
    panic!("{:#?}", ef);
}
