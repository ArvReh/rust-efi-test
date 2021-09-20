use core::ptr::null_mut;

use r_efi::efi;

use crate::arch::serial_writer::SerialWriter;

pub struct CoreGlobals {
    pub efi_system_table: *mut efi::SystemTable,
    pub efi_image_handle: efi::Handle,
    pub serial_writer: *mut SerialWriter 
}

pub static mut CORE_GLOBALS: CoreGlobals = CoreGlobals { efi_system_table: null_mut(), 
                                             efi_image_handle: null_mut(),
                                             serial_writer: null_mut() 
                                           };
