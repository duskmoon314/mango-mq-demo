#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

global_asm!(include_str!("entry.asm"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main() {
    let mut i: usize = 0;
    let peripherals = d1_pac::Peripherals::take().unwrap();
    loop {
        for _ in 0..100000 {
            let _ = 1 + 1;
        }
        i += 1;

        if i % 2 == 0 {
            peripherals
                .GPIO
                .pd_pull1
                .write(|w| w.pd22_pull().pull_down());
        } else {
            peripherals.GPIO.pd_pull1.write(|w| w.pd22_pull().pull_up());
        }
    }
}
