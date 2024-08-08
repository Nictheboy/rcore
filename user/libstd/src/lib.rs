#![no_std]

use core::{
    fmt::{Arguments, Write},
    mem::size_of,
    panic::PanicInfo,
};
use sys::{sys_exit, sys_write};

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn exit(code: usize) -> ! {
    sys_exit(code)
}

pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let ret = write(1, s.as_bytes());
        if ret >= 0 {
            Ok(())
        } else {
            Err(core::fmt::Error)
        }
    }
}

pub fn print(args: Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("Panicked at {}:{}!", location.file(), location.line(),);
    } else {
        println!("Panicked!");
    }
    sys_exit(1);
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        let mut bss_ptr = sbss as *mut usize;
        while bss_ptr < ebss as *mut usize {
            *bss_ptr = 0;
            bss_ptr = bss_ptr.offset(size_of::<usize>() as isize);
        }
    }
}

extern "C" {
    fn main() -> usize;
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    unsafe {
        exit(main());
    }
}
