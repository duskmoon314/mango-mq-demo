#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;
use core::ptr::write_volatile;

global_asm!(include_str!("entry.asm"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main() {
    let mut i: usize = 0;
    loop {
        for _ in 0..100000 {
            let _ = 1 + 1;
        }
        i += 1;
        unsafe {
            if i % 2 == 0 {
                write_volatile(0x020000B8 as *mut u32, 2 << 12);
            } else {
                write_volatile(0x020000B8 as *mut u32, 1 << 12);
            }
        }
    }
}
