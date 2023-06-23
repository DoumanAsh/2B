use core::panic::PanicInfo;

use ufmt_stdio::{ufmt, println};
use esp32c3_hal::esp_riscv_rt::riscv;

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let (file, line) = match info.location() {
        Some(location) => (location.file(), location.line()),
        None => ("UNK", 0),
    };
    let msg = if let Some(msg) = info.payload().downcast_ref::<&str>() {
        msg
    } else {
        "panic"
    };

    println!("FAIL: {} at {}:{}", msg, file, line);
    loop {
        unsafe {
            riscv::asm::wfi();
        }
    }
}

//Just to derive uDebug
#[derive(ufmt::derive::uDebug)]
#[repr(C)]
pub struct TrapFrame {
    pub ra: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s0: usize,
    pub s1: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub gp: usize,
    pub tp: usize,
    pub sp: usize,
    pub pc: usize,
    pub mstatus: usize,
    pub mcause: usize,
    pub mtval: usize,
}

#[export_name = "ExceptionHandler"]
fn exception_handler(context: &TrapFrame) -> ! {
    let mepc = riscv::register::mepc::read();
    let code = riscv::register::mcause::read().code() & 0xff;
    let mtval = riscv::register::mtval::read();

    let code = match code {
        0 => "Instruction address misaligned",
        1 => "Instruction access fault",
        2 => "Illegal instruction",
        3 => "Breakpoint",
        4 => "Load address misaligned",
        5 => "Load access fault",
        6 => "Store/AMO address misaligned",
        7 => "Store/AMO access fault",
        8 => "Environment call from U-mode",
        9 => "Environment call from S-mode",
        10 => "Reserved",
        11 => "Environment call from M-mode",
        12 => "Instruction page fault",
        13 => "Load page fault",
        14 => "Reserved",
        15 => "Store/AMO page fault",
        _ => "UNKNOWN",
    };

    println!("Exception '{}' mepc={:x}, mtval={:x}", code, mepc, mtval);
    println!("{:?}", context);
    loop {
        unsafe {
            riscv::asm::wfi();
        }
    }
}
