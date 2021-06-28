#![no_std]
#![no_main]

#![feature(asm)]
#![feature(abi_efiapi)]



use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn efi_main() {
    unsafe {asm!("mov rax, 0x65");}
    loop {}
}
