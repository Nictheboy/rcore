use core::fmt::{Arguments, Error, Write};
use sbi_rt::Physical;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let num_bytes = bytes.len();
        let ptr = bytes.as_ptr() as usize;
        let phys_addr_lo = ptr & 0xFFFF_FFFF;
        let phys_addr_hi = ptr >> 32;
        let ret = sbi_rt::console_write(Physical::new(num_bytes, phys_addr_lo, phys_addr_hi));
        if ret.is_ok() {
            Ok(())
        } else {
            Err(Error)
        }
    }
}

pub fn print(args: Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
