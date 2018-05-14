#![feature(proc_macro)]
#![no_main]
#![no_std]

#[cfg(feature = "debug")]
extern crate panic_semihosting;
#[cfg(feature = "release")]
extern crate panic_abort;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
#[macro_use(block)]
extern crate nb;
extern crate stm32l4x6_hal as hal;
extern crate cortex_m_rtfm as rtfm;

use rtfm::{app, Threshold};
use hal::gpio::stm32l476vg::led;
use hal::timer;
//use hal::lcd;

mod tasks;

app! {
    device: hal::stm32l4x6,
    resources: {
        static TICK: u64 = 0;
        static LED_RED: led::Led4;
        static LED_TIMER: timer::Timer<hal::stm32l4x6::TIM16>;
        //static LCD: lcd::LCD;
    },
    init: {
        path: tasks::init,
    },

    idle: {
        path: tasks::idle,
    },

    tasks: {
        SYS_TICK: {
            path: tasks::sys_tick,
            resources: [TICK]
        },
        TIM16: {
            path: tasks::toggle,
            resources: [LED_RED, LED_TIMER]
        }
    }
}

fn start() -> ! {
    main();
    unreachable!();
}

entry!(start);

// define the default exception handler
exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}

//Declared by stm32l4x6 with rt feature
// As we are not using interrupts, we just register a dummy catch all handler
//#[link_section = ".vector_table.interrupts"]
//#[used]
//static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];
//
//extern "C" fn default_handler() {
//    asm::bkpt();
//}
