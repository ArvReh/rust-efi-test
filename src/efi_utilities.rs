use core::ptr::null_mut;
use core::ffi::c_void;

use r_efi::efi;

use crate::core_globals::CORE_GLOBALS;

#[inline]
pub unsafe fn efi_exit_boot_services() -> Result<*mut efi::MemoryDescriptor, efi::Status> {
    let mut memorymap_size: usize = 0;
    let mut mapkey: usize = 0;
    let mut descriptor_size: usize = 0;
    let mut descriptor_version: u32 = 0;

    let status = 
        ((*(*CORE_GLOBALS.efi_system_table).boot_services).get_memory_map)(&mut memorymap_size, 
                                                                           null_mut(),
                                                                           &mut mapkey, 
                                                                           &mut descriptor_size,
                                                                           &mut descriptor_version
                                                                           );
    if status != efi::Status::BUFFER_TOO_SMALL { return Err(status); }
    memorymap_size += descriptor_size;
    let mut memorymap: *mut c_void = null_mut();
    let status = 
        ((*(*CORE_GLOBALS.efi_system_table).boot_services).allocate_pool)(efi::LOADER_DATA,
                                                                          memorymap_size,
                                                                          &mut memorymap as *mut *mut c_void
                                                                         );
    if status != efi::Status::SUCCESS { return Err(status) }
    let status = 
        ((*(*CORE_GLOBALS.efi_system_table).boot_services).get_memory_map)(&mut memorymap_size,
                                                                           memorymap as *mut efi::MemoryDescriptor,
                                                                           &mut mapkey, 
                                                                           &mut descriptor_size,
                                                                           &mut descriptor_version
                                                                          );
    if status != efi::Status::SUCCESS { return Err(status) }
    let status = 
        ((*(*CORE_GLOBALS.efi_system_table).boot_services).exit_boot_services)(CORE_GLOBALS.efi_image_handle,
                                                                               mapkey
                                                                              );
    if status != efi::Status::SUCCESS { return Err(status) }
    Ok(null_mut())
}
