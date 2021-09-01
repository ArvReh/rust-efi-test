use core::ptr::null_mut;
use core::ffi::c_void;

use r_efi::efi;

/// Global for the EFI_SYSTEM_TABLE, so it can be used outside the entry point function
pub static mut EFI_SYSTEM_TABLE: *mut efi::SystemTable = null_mut();
/// Global for the EFI_IMAGE_HANDLE, so it can be used outside the entry point function
pub static mut EFI_IMAGE_HANDLE: efi::Handle = null_mut();

pub unsafe fn efi_exit_boot_services() -> efi::Status {
    let mut memorymap_size: usize = 0;
    let mut mapkey: usize = 0;
    let mut descriptor_size: usize = 0;
    let mut descriptor_version: u32 = 0;

    let status = 
        ((*(*EFI_SYSTEM_TABLE).boot_services).get_memory_map)(&mut memorymap_size, 
                                                              null_mut(),
                                                              &mut mapkey, 
                                                              &mut descriptor_size,
                                                              &mut descriptor_version
                                                             );
    if status != efi::Status::BUFFER_TOO_SMALL { return status; }
    memorymap_size += descriptor_size;
    let mut memorymap: *mut c_void = null_mut();
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
    efi::Status::SUCCESS
}
