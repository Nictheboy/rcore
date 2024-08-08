use super::{Arch, FaultHandler, SyscallHandler};
use bios::console::write_str;
use boot::switch_context::switch_to_user;
use trap::trap::trap_init;

mod bios;
mod boot;
mod trap;

pub struct RiscV64;

impl Arch for RiscV64 {
    fn memory_boot_bss() -> &'static mut [u8] {
        extern "C" {
            fn sbss();
            fn ebss();
        }
        unsafe { core::slice::from_raw_parts_mut(sbss as *mut u8, ebss as usize - sbss as usize) }
    }

    fn memory_boot_heap() -> &'static mut [u8] {
        extern "C" {
            fn start_boot_heap();
            fn end_boot_heap();
        }
        unsafe {
            core::slice::from_raw_parts_mut(
                start_boot_heap as usize as *mut u8,
                end_boot_heap as usize - start_boot_heap as usize,
            )
        }
    }

    fn context_switch_to_user(user_pc: usize, user_sp: usize) -> ! {
        unsafe {
            switch_to_user(user_pc, user_sp);
        }
    }

    fn trap_init(syscall_handler: SyscallHandler, fault_handler: FaultHandler) {
        unsafe {
            trap_init(syscall_handler, fault_handler);
        }
    }

    fn debug_print(content: &str) {
        write_str(content);
    }
}
