#![no_std]
#![no_main]

#![feature(asm)]
#![feature(abi_efiapi)]

use arch::mem;

use core::panic::PanicInfo;
use core::ptr;
use core::ffi::c_void;

use r_efi::efi;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        ((*(*EFI_SYSTEM_TABLE).boot_services).exit)(EFI_IMAGE_HANDLE, 
                                                    efi::Status::ABORTED, 
                                                    0, 
                                                    ptr::null_mut()
                                                   );

        loop {} // useless loop because rust doesn't understand that 
                // EFI_SYSTEM_TABLE.Exit will exit the software.
    }
}

/// Global for the EFI_SYSTEM_TABLE, so it can be used outside the entry point function
static mut EFI_SYSTEM_TABLE: *mut efi::SystemTable = ptr::null_mut();
/// Global for the EFI_IMAGE_HANDLE, so it can be used outside the entry point function
static mut EFI_IMAGE_HANDLE: efi::Handle = ptr::null_mut();

#[no_mangle]
pub fn efi_main(image_handle: efi::Handle, system_table: *mut efi::SystemTable) -> efi::Status {
    // Move the efi paramters to their global counterparts.
    unsafe {
        EFI_SYSTEM_TABLE = system_table;
        EFI_IMAGE_HANDLE = image_handle;
    }

    let mut memorymap_size: usize = 0;
    let mut mapkey: usize = 0;
    let mut descriptor_size: usize = 0;
    let mut descriptor_version: u32 = 0;

    unsafe {
        let status = 
            ((*(*EFI_SYSTEM_TABLE).boot_services).get_memory_map)(&mut memorymap_size, 
                                                                  ptr::null_mut(),
                                                                  &mut mapkey, 
                                                                  &mut descriptor_size,
                                                                  &mut descriptor_version
                                                                 );
        if status != efi::Status::BUFFER_TOO_SMALL { return status; }
        memorymap_size += descriptor_size;
        let mut memorymap: *mut c_void = ptr::null_mut();
        let status = 
            ((*(*EFI_SYSTEM_TABLE).boot_services).allocate_pool)(efi::LOADER_DATA,
                                                                 memorymap_size,
                                                                 &mut memorymap as *mut *mut c_void
                                                                );
        if status != efi::Status::SUCCESS { return status }
        let status = 
            ((*(*EFI_SYSTEM_TABLE).boot_services).get_memory_map)(&mut memorymap_size,
                                                                  memorymap as *mut efi::MemoryDescriptor,
                                                                  &mut mapkey, 
                                                                  &mut descriptor_size,
                                                                  &mut descriptor_version
                                                                 );
        if status != efi::Status::SUCCESS { return status }
        let status = 
            ((*(*EFI_SYSTEM_TABLE).boot_services).exit_boot_services)(EFI_IMAGE_HANDLE,
                                                                      mapkey
                                                                     );
        if status != efi::Status::SUCCESS { return status }
        loop {}
    }
    efi::Status::NOT_READY 
}
