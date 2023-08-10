#![no_std]
#![no_main]

mod pl011;
mod logger;
mod start;

use log::{info, error};

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    info!("Hello world!");
    core::arch::asm!("hvc #0", in("x0") 0x84000008u64);
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("Panic: {}", info);
    unsafe { core::arch::asm!("hvc #0", in("x0") 0x84000008u64); }
    loop {}
}
