use core::panic::PanicInfo;

#[cfg(not(debug_assertions))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    //we abort on panic
    loop {}
}

#[cfg(debug_assertions)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use cortex_m::asm;

    log!("Panic: {}", info);

    asm::bkpt();

    loop {}
}
