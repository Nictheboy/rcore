#![no_std]
#![no_main]

use core::arch::{asm, global_asm};

use trap::trap_init;
mod console;
mod lang_items;
mod trap;

global_asm!(include_str!("entry.asm"));

static mut USER_STACK: [i8; 4096 * 16] = [0; 4096 * 16];

#[no_mangle]
fn main() -> ! {
    printkln!("Booting...");
    clear_bss();
    trap_init();
    printkln!("Boot finished");
    unsafe {
        switch_to_user(0x80400000, USER_STACK.as_ptr() as usize);
    }
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

fn switch_to_user(user_pc: usize, user_sp: usize) -> ! {
    let mut sstatus: usize;
    unsafe {
        asm!("csrr {sstatus}, sstatus", sstatus = out(reg) sstatus);
    }
    // Set SPP to 0 (user mode), as if we are returning from an user trap
    sstatus &= !(1 << 8);
    unsafe {
        asm!(
            "csrw sepc, {user_pc}",
            "csrw sstatus, {sstatus}",
            "csrw sscratch, {user_sp}",
            "csrrw sp, sscratch, sp",
            "sret",
            user_pc = in(reg) user_pc,
            sstatus = in(reg) sstatus,
            user_sp = in(reg) user_sp,
        );
    }
    panic!(
        "Control is already switched to user space, but it switched back to the original function"
    );
}
