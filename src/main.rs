#![no_std]
#![no_main]

#![feature(asm)]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn efi_main() {
    let i: i8 = 5;
    unsafe {asm!("mov rax, 0x55");}
    loop {}
}
