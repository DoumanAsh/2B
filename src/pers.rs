use hal::gpio::stm32l476vg::led::{Led4, Led5};
use hal::common::Constrain;
use hal::rcc::{clocking, Clocks};
use hal::time;
use hal::serial::{self, RawSerial, Serial};
use hal::crc::CRC;

mod gpio {
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

///Contains available leds on device
pub struct Led {
    pub green: Led5,
    pub red: Led4
}

impl Led {
    //Congifure LEDs
    fn new(ahb: &mut hal::rcc::AHB) -> Self {
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

pub type Serial1 = Serial<hal::serial::USART1, gpio::PB6<gpio::AF7>, gpio::PB7<gpio::AF7>, hal::serial::DummyPin>;
pub struct Device {
    pub led: Led,
    pub clocks: Clocks,
    pub serial1: Serial1,
    pub crc: CRC,
}

impl Device {
    pub fn init(device: hal::stm32l4x6::Peripherals) -> Self {
        let mut rcc = device.RCC.constrain();
        let mut flash = device.FLASH.constrain();
        let clocks = rcc.cfgr.sysclk(clocking::SysClkSource::MSI(clocking::MediumSpeedInternalRC::new(32_000_000, false)))
                             .hclk(time::MegaHertz(32))
                             .pclk1(time::MegaHertz(32))
                             .pclk2(time::MegaHertz(32))
                             .freeze(&mut flash.acr);

        let serial1 = {
            let mut gpio = gpio::B::new(&mut rcc.ahb);
            let tx = gpio.PB6.into_alt_fun::<gpio::AF7>(&mut gpio.moder, &mut gpio.afrl);
            let rx = gpio.PB7.into_alt_fun::<gpio::AF7>(&mut gpio.moder, &mut gpio.afrl);
            Serial::new(device.USART1, (tx, rx, hal::serial::DummyPin), 115_200, &clocks, &mut rcc.apb2)
        };

        serial1.subscribe(serial::Event::Rxne);

        CRC::enable(&mut rcc.ahb);

        Self {
            led: Led::new(&mut rcc.ahb),
            clocks,
            serial1,
            crc: CRC::new(device.CRC),
        }
    }
}
