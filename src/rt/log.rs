use ::core::hint;

use ::cortex_m_log::printer;

type LoggerType = printer::semihosting::InterruptFree<printer::semihosting::hio::HStdout>;

static mut LOGGER: Option<LoggerType> = None;

pub fn logger() -> &'static mut LoggerType {
    unsafe {
        match LOGGER.as_mut() {
            Some(logger) => logger,
            None => hint::unreachable_unchecked(),
        }
    }
}

pub fn set_logger() {
    let logger = LoggerType::stdout().unwrap();
    unsafe {
        LOGGER = Some(logger);
    }
}
