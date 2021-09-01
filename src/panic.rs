use core::panic::PanicInfo;
use core::ptr;

use r_efi::efi;

use crate::arch::utils::asm_loop;
use crate::efi_utilities::{EFI_SYSTEM_TABLE, EFI_IMAGE_HANDLE};

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        if !(*EFI_SYSTEM_TABLE).boot_services.is_null() {
            ((*(*EFI_SYSTEM_TABLE).boot_services).exit)(EFI_IMAGE_HANDLE, 
                                                        efi::Status::ABORTED, 
                                                        0, 
                                                        ptr::null_mut()
                                                       );
        } else {
            asm_loop();
        }

        loop {} // useless loop because rust doesn't understand that 
                // EFI_SYSTEM_TABLE.Exit() will exit the software.
    }
}
