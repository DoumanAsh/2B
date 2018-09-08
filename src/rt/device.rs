use ::hal::stm32l4x6::Peripherals as DevicePeripherals;
use ::hal::common::Constrain;
use ::hal::rcc::{clocking, AHB};

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
    let pers = DevicePeripherals::take().unwrap();
    let mut rcc = pers.RCC.constrain();

    let mut flash = pers.FLASH.constrain();
    let hsi16 = clocking::HighSpeedInternal16RC {
        always_on: true,
        auto_start: true,
    };
    let clocks = rcc.cfgr.hclk(80_000).sysclk(clocking::SysClkSource::HSI16(hsi16)).freeze(&mut flash.acr);

    let led = Led::new(&mut rcc.ahb);

    Device {
        led
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
    pub led: Led
}
