#![no_std]
#![no_main]

use core::arch::global_asm;
mod console;
mod lang_items;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn main() -> ! {
    println!("Hello, world!");
    panic!("Kernel main reached the end!");
}
