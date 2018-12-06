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
    error!("Panic: {}", info);

    loop {}
}
