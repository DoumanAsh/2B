use super::*;

extern crate embedded_hal;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;

use ::hal::common::Constrain;
use ::hal::gpio::stm32l476vg;
use ::hal::timer;
use ::led::{Led4, Led5};
//use ::lcd;

///Runs at the start and is capable of initializing `init::LateResource`
pub fn init(mut p: init::Peripherals, _r: init::Resources) -> init::LateResources {
    use self::cortex_m::peripheral::syst::SystClkSource;

    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();
    let mut _pwr = p.device.PWR.constrain();
    //Configre Clocks
    let clocks = rcc.cfgr.hclk(40_000).freeze(&mut flash.acr);
    //Configure  timer
    let mut timer = timer::Timer::tim16(p.device.TIM16, 10, clocks, &mut rcc.apb2);
    timer.subscribe(timer::Event::Timeout);

    //Configure source for SYST
    p.core.SYST.set_clock_source(SystClkSource::Core);
    p.core.SYST.enable_interrupt();
    p.core.SYST.enable_counter();

    //Congifure LEDs
    let mut gpio = stm32l476vg::gpio::E::new(&mut rcc.ahb);
    let mut led = Led5::new(gpio.PE8.into_push_pull_output(&mut gpio.moder, &mut gpio.otyper));
    led.on();
    let mut gpio = stm32l476vg::gpio::B::new(&mut rcc.ahb);
    let led = gpio.PB2.into_push_pull_output(&mut gpio.moder, &mut gpio.otyper);
    let led = Led4::new(led);

    //Configre LCD
    //let lcd = {
    //    lcd::LCD::init_lse(&mut rcc.apb1, &mut rcc.ahb, &mut pwr, &mut rcc.bdcr);

    //    let mut config = lcd::config::Config::default();
    //    config.prescaler = Some(lcd::config::Prescaler::PS_64);
    //    config.divider = Some(lcd::config::Divider::DIV_17);
    //    config.duty = Some(lcd::config::Duty::Static);
    //    config.bias = Some(lcd::config::Bias::Bias13);
    //    config.contrast = Some(lcd::config::Contrast::Five);

    //    match lcd::LCD::validate(&mut p.device.LCD, &mut rcc.bdcr, &config) {
    //        lcd::ValidationResult::Ok(_) => lcd::LCD::new(p.device.LCD, config),
    //        lcd::ValidationResult::SmallFrameRate => panic!("Resulting framerate is too small"),
    //        lcd::ValidationResult::BigFrameRate => panic!("Resulting framerate is too big"),
    //        lcd::ValidationResult::ClockNotSet => panic!("Clock is not set for LCD")
    //    }
    //};

    init::LateResources {
        LED_RED: led,
        LED_TIMER: timer,
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
