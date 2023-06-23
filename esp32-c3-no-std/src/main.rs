#![no_std]
#![no_main]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![cfg_attr(rustfmt, rustfmt_skip)]

//ESP32-C3 UART
#[no_mangle]
static ESP_UART_ADDR: usize = 0x40000068;

use esp32c3_hal::entry;
use esp32c3_hal::esp_riscv_rt::riscv;
use esp32c3_hal::macros::interrupt;
use esp32c3_hal::prelude::*; //All these stupid traits are inside prelude

use ufmt_stdio::{ufmt, println};

macro_rules! unreach {
    () => {
        unsafe {
            core::hint::unreachable_unchecked()
        }
    }
}

///Awaits for Infallible operation to complete
macro_rules! block_infallible {
    ($op:expr) => {
        loop {
            match $op {
                Ok(res) => break res,
                Err(esp32c3_hal::nb::Error::WouldBlock) => core::hint::spin_loop(),
                Err(core::convert::Infallible) => unreach!(),
            }
        }
    };
}

mod panic;
mod peripherals;

use peripherals::Timestamp;

#[entry]
fn main() -> ! {
    peripherals::init();

    println!("Start");

    loop {
        unsafe {
            riscv::asm::wfi();
        }
    }
}

#[interrupt]
fn TG0_T0_LEVEL() {
    let peripherals = peripherals::instance();
    peripherals.timer0.with(|mut timer0| {
        timer0.clear_interrupt();
        println!("[{}]: Timer0", Timestamp::uptime());
        timer0.start(1u32.secs());
    });
}
