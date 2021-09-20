use core::panic::PanicInfo;
use core::ptr;

use r_efi::efi;

use crate::arch::utils::asm_loop;
use crate::core_globals::CORE_GLOBALS;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        if !(*CORE_GLOBALS.efi_system_table).boot_services.is_null() {
            ((*(*CORE_GLOBALS.efi_system_table).boot_services).exit)(CORE_GLOBALS.efi_image_handle, 
                                                                     efi::Status::ABORTED, 
                                                                     0, 
                                                                     ptr::null_mut()
                                                                    );
        } else {
            asm_loop();
        }

        loop {} // useless loop because rust doesn't understand that 
                // this line is unreachable;
    }
}
