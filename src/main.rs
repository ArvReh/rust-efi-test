#![no_std]
#![no_main]

#![feature(asm)]
#![feature(abi_efiapi)]

use core::ptr;
use core::ffi::c_void;
use core::fmt::Write;

use r_efi::efi;

use kernel::panic::panic;
use kernel::arch::mem;
use kernel::efi_utilities;
use kernel::arch::serial_writer::SerialWriter;
use kernel::print;
use kernel::core_globals::CORE_GLOBALS;

struct Test {
    pub writer: *mut SerialWriter
}

#[no_mangle]
pub fn efi_main(image_handle: efi::Handle, system_table: *mut efi::SystemTable) -> efi::Status {
    unsafe {
        // Move the efi paramters to their global counterparts.
        CORE_GLOBALS.efi_system_table = system_table;
        CORE_GLOBALS.efi_image_handle = image_handle;
        
        let mut efi_memory_map: *mut efi::MemoryDescriptor = ptr::null_mut();
        match efi_utilities::efi_exit_boot_services() {
            Ok(ok) => efi_memory_map = ok,
            Err(err) => return err
        };

        
        let writer = &mut SerialWriter::init(0x3F8) as *mut SerialWriter;
        //let serial = test.writer;
        let string: &str = "Hello, World!";
        (*writer).clear();
        (*writer).write_str(string);
        (*writer).write_char('.');
        //print!("{}", "Hello, World!");
    }

    panic!();
}
