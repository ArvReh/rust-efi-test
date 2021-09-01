use crate::tty;

static COM_PORT: u16 = 0x3F8;

#[inline(always)]
unsafe fn outb(port: u16, byte: u8) {
    asm!("out dx, al",
         in("dx") port,
         in("al") byte);
}
#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let byte: u8;
    asm!("in al, dx",
         in("dx") port,
         out("al") byte);
    byte

}

pub struct SerialTty(u16);

impl SerialTty {
    pub fn init() -> Self {
        unsafe {
            outb(COM_PORT+1, 0x00); // Disable interrupts
            outb(COM_PORT+3, 0x80); // Set DLAB bit
            outb(COM_PORT,   0x03); // Write LSB of baud rate
            outb(COM_PORT+1, 0x00); // Write MSB of baud rate
            outb(COM_PORT+3, 0x03); // Set work length to 8b

            outb(COM_PORT+4, 0x1E); // Set loopback mode
            outb(COM_PORT,   0xDE); // Write test data
            if inb(COM_PORT) != 0xDE {
                panic!();
            }
            outb(COM_PORT+4, 0x0F); // Disable loopback mode
        }
        return SerialTty(COM_PORT)
    }
}

impl tty::TtySink for SerialTty {
    fn writeb(&self, byte: u8) {
        unsafe {
            outb(self.0, byte);
        }
    }
    fn writes(&self, string: *const u8) {
        unsafe {
            asm!("outsb",
                 in("dx") self.0,
                 in("rsi") string);
        }
    }
}
