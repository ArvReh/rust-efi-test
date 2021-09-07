use core::fmt;

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
    fn write_str(&mut self, s: &str) -> fmt::Result;
    fn write_char(&mut self, c: char) -> fmt::Result;
    fn clear(&mut self) -> fmt::Result;
}
