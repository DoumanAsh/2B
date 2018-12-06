extern crate cortex_m_log;

use ::core::hint;

use self::cortex_m_log::printer;

type PrinterType = printer::semihosting::InterruptFree<printer::semihosting::hio::HStdout>;
type LoggerType = cortex_m_log::log::Logger<PrinterType>;

static mut LOGGER: Option<LoggerType> = None;

pub fn get_logger() -> &'static mut LoggerType {
    unsafe {
        match LOGGER.as_mut() {
            Some(logger) => logger,
            None => hint::unreachable_unchecked(),
        }
    }
}

pub fn set_logger() {
    let logger = PrinterType::stdout().unwrap();
    let logger = LoggerType {
        inner: logger,
        level: log::LevelFilter::Info
    };
    unsafe {
        LOGGER = Some(logger);
    }

    let _ = cortex_m_log::log::init(get_logger());
}
