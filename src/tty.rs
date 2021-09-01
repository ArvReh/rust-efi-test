pub struct Tty<T: TtySink> {
    pub sink: T
}

impl<T: TtySink> Tty<T> {
    pub fn new(t: T) -> Self {
        Tty {
            sink: t
        }
    }
}

pub trait TtySink {
    //fn read() -> u8;
    /// # Write byte
    /// Writes a single byte to the terminal. `\n` produces
    ///   undefined behaviour.
    fn writeb(&self, byte: u8);
    /// # Write string
    /// Writes a single byte to the terminal. `\n` produces
    ///   a newline.
    fn writes(&self, string: *const u8);
    //fn clear();
}
