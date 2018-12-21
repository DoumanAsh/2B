#![no_main]
#![no_std]

extern crate stm32l4x6_hal as hal;

use rtfm::app;
use log::{error, warn, info};
use cortex_m_log::printer;
use cortex_m_rt::{exception};
use hal::embedded_hal::serial::{Read, Write};
use hal::embedded_hal::digital::ToggleableOutputPin;
use hal::nb;

type PrinterType = printer::semihosting::InterruptFree<printer::semihosting::hio::HStdout>;
type LoggerType = cortex_m_log::log::Logger<PrinterType>;

mod pers;
mod panic;
mod utils;
mod counters;

use self::utils::ResultExt;
use self::counters::Counters;

const PERIOD: u32 = 8_000_000;

#[app(device = hal::stm32l4x6)]
const APP: () = {
    static mut COUNTERS: Counters = Counters::new();

    static mut LED: pers::Led = ();
    static mut SERIAL1: pers::Serial1 = ();

    #[init(schedule = [working])]
    fn init() {
        static mut LOGGER: Option<LoggerType> = None;

        #[cfg(debug_assertions)]
        {
            let logger = PrinterType::stdout().unwrap();
            let logger = LoggerType {
                inner: logger,
                level: log::LevelFilter::Info
            };
            *LOGGER = Some(logger);
            let _ = match LOGGER {
                Some(ref mut logger) => cortex_m_log::log::init(logger),
                None => unreach!(),
            };
        }

        info!("Init application");

        let mut device = pers::Device::init(device);
        device.led.green.on();

        const WELCOME: &'static [u8; 13] = b"Hello world!\n";

        for byte in WELCOME {
            match nb::block!(device.serial1.write(*byte)) {
                Ok(_) => (),
                Err(error) => error!("Error while writing welcome: {:?}", error),
            }
        }

        match nb::block!(device.serial1.flush()) {
            Ok(_) => (),
            Err(error) => error!("Error while flushing welcome: {:?}", error),
        }

        match nb::block!(device.serial1.read()) {
            Ok(_) => info!("Got byte"),
            Err(error) => error!("Error while reading: {:?}", error),
        }

        schedule.working(rtfm::Instant::now() + PERIOD.cycles()).unreach_err();

        LED = device.led;
        SERIAL1 = device.serial1;
    }

    #[idle(resources = [COUNTERS])]
    fn idle() -> ! {
        info!("Start application");
        loop {
            cortex_m::asm::wfe();
            resources.COUNTERS.interrupt += 1;
        }
    }

    #[task(resources = [LED], schedule = [working])]
    fn working() {
        resources.LED.green.toggle();
        schedule.working(scheduled + PERIOD.cycles()).unreach_err();
    }

    #[interrupt(resources = [SERIAL1])]
    fn USART1() {
        //WHY FRAMING ERROR FOR FUCK SAKE!?
        //match resources.SERIAL1.read() {
        //    Ok(_) => info!("Got byte"),
        //    Err(error) => error!("Serial error: {:?}" , error),
        //}
    }

    extern "C" {
        fn LCD();
    }
};

#[allow(unused)]
#[exception]
fn DefaultHandler(irqn: i16) {
    warn!("DefaultHandler: IRQn = {}", irqn);
}

#[exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    // prints the exception frame as a panic message
    panic!("{:#?}", ef);
}
