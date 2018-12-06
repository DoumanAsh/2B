#![no_main]
#![no_std]

extern crate stm32l4x6_hal as hal;
extern crate cortex_m;
extern crate cortex_m_rt;
#[cfg(debug_assertions)]
#[macro_use]
extern crate log;
extern crate embedded_hal;
extern crate nb;

use hal::serial::RawSerial;
use embedded_hal::serial::{Read, Write};
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
    const WELCOME: &'static [u8; 13] = b"Hello world!\n";
    unsafe {
        let mut rt = rt::init();
        rt.device.serial.subscribe(hal::serial::Event::Rxne);
        rt.device.serial.subscribe(hal::serial::Event::Txe);

        for byte in WELCOME {
            match nb::block!(rt.device.serial.write(*byte)) {
                Ok(_) => (),
                Err(error) => error!("Error while writing welcome: {:?}", error),
            }
        }

        RT = Some(rt);
    }
}

#[entry]
fn main() -> ! {
    init();

    info!("Initialize firmware");
    // infinite loop; just so we don't leave this stack frame
    loop {
        wfe();
        let rt = get_rt();
        match nb::block!(rt.device.serial.read()) {
            Ok(byte) => match nb::block!(rt.device.serial.write(byte)) {
                Ok(_) => (),
                Err(error) => error!("Error writing: {:?}", error),
            },
            Err(error) => error!("Error reading: {:?}", error),
        }
    }
}

#[allow(unused)]
#[exception]
fn DefaultHandler(irqn: i16) {
    warn!("DefaultHandler: IRQn = {}", irqn);
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
