use core::arch::global_asm;

pub mod switch_context;
global_asm!(include_str!("entry.asm"));
