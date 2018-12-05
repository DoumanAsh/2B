use ::hal::stm32l4x6::Peripherals as DevicePeripherals;
use ::hal::common::Constrain;
use ::hal::rcc::{clocking, AHB, Clocks};
use ::hal::serial::Serial;
use ::hal::time;
//use ::hal::lcd;

mod gpio {
    //Serial1
    pub use ::hal::gpio::{
        PB6, PB7, PB5, AF7
    };

    pub use ::hal::gpio::stm32l476vg::gpio::{
        E, PE8,
        B, PB2,
    };

    pub use ::hal::gpio::{
        PushPull
    };
}
use hal::gpio::stm32l476vg::led::{Led4, Led5};

pub fn init() -> Device {
    let mut pers = DevicePeripherals::take().unwrap();
    let mut rcc = pers.RCC.constrain();
    let mut pwr = pers.PWR.constrain();

    let mut flash = pers.FLASH.constrain();
    let clocks = rcc.cfgr.sysclk(clocking::SysClkSource::MSI(clocking::MediumSpeedInternalRC::new(32_000_000, false)))
                         .hclk(time::MegaHertz(32))
                         .pclk2(time::MegaHertz(32))
                         .freeze(&mut flash.acr);

    let led = Led::new(&mut rcc.ahb);

    let serial = {
        let mut gpio = gpio::B::new(&mut rcc.ahb);
        let tx = gpio.PB6.into_alt_fun::<gpio::AF7>(&mut gpio.moder, &mut gpio.afrl);
        let rx = gpio.PB7.into_alt_fun::<gpio::AF7>(&mut gpio.moder, &mut gpio.afrl);
        let ck = gpio.PB5.into_alt_fun::<gpio::AF7>(&mut gpio.moder, &mut gpio.afrl);
        Serial::new(pers.USART1, (tx, rx, ck), 115_200.into(), &clocks, &mut rcc.apb2)
    };

    //Configre LCD
    //let mut screen = {
    //    lcd::LCD::init_lse(&mut rcc.apb1, &mut rcc.ahb, &mut pwr, &mut rcc.bdcr);

    //    let mut config = lcd::config::Config::default();
    //    config.prescaler = Some(lcd::config::Prescaler::PS_64);
    //    config.divider = Some(lcd::config::Divider::DIV_17);
    //    config.duty = Some(lcd::config::Duty::Static);
    //    config.bias = Some(lcd::config::Bias::Bias13);
    //    config.contrast = Some(lcd::config::Contrast::Five);

    //    match lcd::LCD::validate(&mut pers.LCD, &mut rcc.bdcr, &config) {
    //        lcd::ValidationResult::Ok(_) => lcd::LCD::new(pers.LCD, config),
    //        lcd::ValidationResult::SmallFrameRate => panic!("Resulting framerate is too small"),
    //        lcd::ValidationResult::BigFrameRate => panic!("Resulting framerate is too big"),
    //        lcd::ValidationResult::ClockNotSet => panic!("Clock is not set for LCD")
    //    }
    //};

    //screen.write_ram::<lcd::ram::index::Zero>(0xffffffff);
    //screen.write_ram::<lcd::ram::index::One>(0xffffffff);
    //screen.write_ram::<lcd::ram::index::Two>(0xffffffff);
    //screen.write_ram::<lcd::ram::index::Three>(0xffffffff);
    //screen.write_ram::<lcd::ram::index::Four>(0xffffffff);
    //screen.write_ram::<lcd::ram::index::Five>(0xffffffff);
    //screen.write_ram::<lcd::ram::index::Six>(0xffffffff);
    //screen.write_ram::<lcd::ram::index::Seven>(0xffffffff);
    //screen.update_request();

    Device {
        led,
        clocks,
        serial
    }
}

///Contains available leds on device
pub struct Led {
    pub green: Led5,
    pub red: Led4
}

impl Led {
    //Congifure LEDs
    fn new(ahb: &mut AHB) -> Self {
        let mut gpio = gpio::E::new(ahb);
        let mut green = Led5::new(gpio.PE8.into_output::<gpio::PushPull>(&mut gpio.moder, &mut gpio.otyper));
        green.on();
        let mut gpio = gpio::B::new(ahb);
        let red = gpio.PB2.into_output::<gpio::PushPull>(&mut gpio.moder, &mut gpio.otyper);
        let red = Led4::new(red);

        Self {
            green,
            red
        }
    }
}

pub struct Device {
    pub led: Led,
    pub clocks: Clocks,
    pub serial: Serial<::hal::serial::USART1, gpio::PB6<gpio::AF7>, gpio::PB7<gpio::AF7>, gpio::PB5<gpio::AF7>>,
}
