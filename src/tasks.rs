use super::*;

extern crate embedded_hal;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;

use ::hal::common::Constrain;
use ::hal::gpio::stm32l476vg;
use ::hal::timer;
use ::led::{Led4, Led5};

///Runs at the start and is capable of initializing `init::LateResource`
pub fn init(mut p: init::Peripherals, _r: init::Resources) -> init::LateResources {
    use self::cortex_m::peripheral::syst::SystClkSource;

    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer = timer::Timer::tim16(p.device.TIM16, 10, clocks, &mut rcc.apb2);
    timer.subscribe(timer::Event::Timeout);

    p.core.SYST.set_clock_source(SystClkSource::Core);
    p.core.SYST.enable_interrupt();
    p.core.SYST.enable_counter();

    let mut gpio = stm32l476vg::gpio::E::new(&mut rcc.ahb);
    let mut led = Led5::new(gpio.PE8.into_push_pull_output(&mut gpio.moder, &mut gpio.otyper));
    led.on();
    let mut gpio = stm32l476vg::gpio::B::new(&mut rcc.ahb);
    let led = gpio.PB2.into_push_pull_output(&mut gpio.moder, &mut gpio.otyper);
    let led = Led4::new(led);

    init::LateResources {
        LED_RED: led,
        LED_TIMER: timer
    }
}
///Kinda main loop that should not exit.
///For this reason it waits for interrupts.
pub fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

pub fn sys_tick(_t: &mut Threshold, mut r: SYS_TICK::Resources) {
    *r.TICK = (*r.TICK).overflowing_add(1).0;
}

pub fn toggle(_t: &mut Threshold, mut r: TIM16::Resources) {
    let timer = &mut *r.LED_TIMER;
    timer.reset_overflow();

    let led = &mut *r.LED_RED;
    match led.is_off() {
        true => led.on(),
        false => led.off()
    }

    //TODO: in optimization mode I need to reset twice...
    timer.reset_overflow();
}
