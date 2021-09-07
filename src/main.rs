#![no_std]
#![no_main]

#![feature(asm)]
#![feature(abi_efiapi)]

use kernel::panic::panic;
use kernel::arch::mem;
use kernel::efi_utilities::{EFI_SYSTEM_TABLE, EFI_IMAGE_HANDLE, efi_exit_boot_services};
use kernel::arch::serial_tty::SerialTty;
use kernel::tty::TtySink;
use kernel::tty;

use core::ptr;
use core::ffi::c_void;

use r_efi::efi;

#[no_mangle]
pub fn efi_main(image_handle: efi::Handle, system_table: *mut efi::SystemTable) -> efi::Status {
    unsafe {
        // Move the efi paramters to their global counterparts.
        EFI_SYSTEM_TABLE = system_table;
        EFI_IMAGE_HANDLE = image_handle;

        let status: efi::Status = efi_exit_boot_services();
        if status != efi::Status::SUCCESS { return status; };

        let mut terminal: tty::Tty<SerialTty> = tty::Tty::new(SerialTty::init(0x3F8));
        let string: &str = "Hello, World!";
        terminal.sink.clear();
        terminal.sink.write_str(string);
        terminal.sink.write_char('.');
        if status != efi::Status::SUCCESS { return status; };
    }

    panic!();
}
