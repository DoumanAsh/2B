use ::core::hint;

use ::cortex_m::peripheral as core_pers;

mod device;
pub mod tick;
#[cfg(feature = "debug")]
pub mod log;

#[cfg(feature = "debug")]
#[macro_export]
macro_rules! log {
    ($($arg:tt)+) => ({
        use ::cortex_m_log::printer::Printer;
        ::rt::log::logger().println(format_args!($($arg)+));
    })
}

#[cfg(feature = "release")]
#[macro_export]
macro_rules! log {
    ($($arg:tt)+) => ({
    })
}

pub struct Guard {
    pub dcb: core_pers::DCB,
    pub syst: core_pers::SYST,
    pub device: device::Device
}

impl Drop for Guard {
    fn drop(&mut self) {
    }
}

pub fn init() -> Guard {
    let mut pers = match core_pers::Peripherals::take() {
        Some(pers) => pers,
        None => unsafe { hint::unreachable_unchecked() }
    };

    #[cfg(feature = "debug")]
    {
        log::set_logger();
    }

    let device = device::init();

    // Enable SysTick
    pers.SYST.set_clock_source(core_pers::syst::SystClkSource::Core);
    let freq = core_pers::SYST::get_ticks_per_10ms();
    log!("Set SysTick frequency={}", freq);
    pers.SYST.set_reload(freq);
    pers.SYST.clear_current();
    pers.SYST.enable_counter();
    pers.SYST.enable_interrupt();

    Guard {
        dcb: pers.DCB,
        syst: pers.SYST,
        device: device,
    }
}
