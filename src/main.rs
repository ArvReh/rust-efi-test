#![no_std]
#![no_main]

#![feature(asm)]
#![feature(abi_efiapi)]

use kernel::panic::panic;
use kernel::arch::mem;
use kernel::efi_utilities::{EFI_SYSTEM_TABLE, EFI_IMAGE_HANDLE, efi_exit_boot_services};
//use kernel::arch::serial_tty::SerialTty;
use kernel::tty;
use kernel::tty::TtySink;

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

        ptr::write_volatile(0xb8000 as *mut u16, 0x4F4F);
        //let terminal: tty::Tty<SerialTty> = tty::Tty::new(SerialTty::init());
        //let cstr: [u8; 6] = [0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x00];
        //terminal.sink.writes(cstr.as_ptr());
        //if status != efi::Status::SUCCESS { return status; };
    }

    panic!();
}
