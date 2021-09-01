pub unsafe fn asm_loop() -> ! {
    loop { asm!("nop") } 
}
