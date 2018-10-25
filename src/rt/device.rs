use ::hal::stm32l4x6::Peripherals as DevicePeripherals;
use ::hal::common::Constrain;
use ::hal::rcc::{clocking, AHB, Clocks};
//use ::hal::lcd;

mod gpio {
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
    let hsi16 = clocking::HighSpeedInternal16RC {
        always_on: true,
        auto_start: true,
    };
    let clocks = rcc.cfgr.hclk(80_000).sysclk(clocking::SysClkSource::HSI16(hsi16)).freeze(&mut flash.acr);

    let led = Led::new(&mut rcc.ahb);

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
        clocks
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
}
