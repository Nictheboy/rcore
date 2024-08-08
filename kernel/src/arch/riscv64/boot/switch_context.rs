use core::arch::asm;

pub unsafe fn switch_to_user(user_pc: usize, user_sp: usize) -> ! {
    let mut sstatus: usize;
    asm!("csrr {sstatus}, sstatus", sstatus = out(reg) sstatus);
    // Set SPP to 0 (user mode), as if we are returning from an user trap
    sstatus &= !(1 << 8);
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
    panic!(
        "Control is already switched to user space, but it switched back to the original function"
    );
}
