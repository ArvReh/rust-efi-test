use core::fmt;
use core::str;
use core::mem;

use crate::tty;
use crate::tty::TtySink;

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
    pub fn init(port: u16) -> Self {
        let tty = SerialTty(port);
        unsafe {
            outb(tty.0+1, 0x00); // Disable interrupts
            outb(tty.0+3, 0x80); // Set DLAB bit
            outb(tty.0,   0x03); // Write LSB of baud rate
            outb(tty.0+1, 0x00); // Write MSB of baud rate
            outb(tty.0+2, 0x83); // Set FIFO mode
            outb(tty.0+3, 0x03); // Set work length to 8b

            outb(tty.0+4, 0x1E); // Set loopback mode
            outb(tty.0,   0xDE); // Write test data
            if inb(tty.0) != 0xDE {
                panic!();
            }
            outb(tty.0+4, 0x0F); // Disable loopback mode
        }
        tty
    }
}

impl TtySink for SerialTty {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            for c in s.bytes() {
                //while inb(self.0+5)&0x20 != 0 {}
                outb(self.0, c);
            }
        }
        Ok(())
    }
    fn write_char(&mut self, c: char) -> fmt::Result {
        unsafe {
            for b in mem::transmute::<char, [u8; 4]>(c).iter() {
                outb(self.0, *b);
            }
        }
        Ok(())
    }

    fn clear(&mut self) -> fmt::Result {
        let clear_str: &str = match str::from_utf8(&[0x1B, 0x5B, 0x32, 0x4A, 0x1B,
                                                     0x5B, 0x0, 0x3B, 0x0, 0x48]) 
        {
            Ok(ok) => ok,
            Err(_) => return Err(fmt::Error)
        };
        self.write_str(&clear_str)?;
        Ok(())
    }
}
