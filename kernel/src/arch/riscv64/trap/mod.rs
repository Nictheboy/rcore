use core::arch::global_asm;

pub mod trap;
global_asm!(include_str!("trap.asm"));
