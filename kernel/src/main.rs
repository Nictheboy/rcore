#![no_std]
#![no_main]

extern crate alloc;
extern crate palloc;
use core::ptr::NonNull;

use arch::{Arch, CurrentArch};
use lang_items::ALLOCATOR;
use palloc::GlobalPalloc;
use syscall::syscall_handler;

mod arch;
mod console;
mod lang_items;
mod syscall;

#[no_mangle]
fn main() -> ! {
    printkln!("Booting...");
    clear_bss();
    init_kalloc();
    CurrentArch::trap_init(syscall_handler, fault_handler);
    printkln!("Boot finished");
    CurrentArch::context_switch_to_user(0x80400000, 0x80600000);
}

fn clear_bss() {
    let bss = CurrentArch::memory_boot_bss();
    for i in bss.iter_mut() {
        *i = 0;
    }
}

fn init_kalloc() {
    let heap = CurrentArch::memory_boot_heap();
    unsafe { ALLOCATOR.init(NonNull::new(heap.as_mut_ptr()).unwrap(), heap.len()) };
}

fn fault_handler(fault: arch::Fault) {
    match fault {
        arch::Fault::Unknown(desc) => printkln!("Unknown fault: {}", desc),
        arch::Fault::MemoryFault(addr) => printkln!("Memory fault: addr={:#x}", addr),
        arch::Fault::IllegalInstruction => printkln!("Illegal instruction"),
        arch::Fault::Breakpoint => printkln!("Breakpoint"),
    }
    panic!("Fault");
}
