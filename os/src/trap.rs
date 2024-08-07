use crate::printkln;
use core::arch::{asm, global_asm};

global_asm!(include_str!("trap.asm"));

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: usize,
    pub sepc: usize,
}

pub fn trap_init() {
    extern "C" {
        fn _trap_handler();
    }
    unsafe {
        asm!(
            "csrw stvec, {_trap_handler}",
            _trap_handler = in(reg) _trap_handler,
        );
    }
}

#[no_mangle]
pub fn trap_handler(_trap_context: &mut TrapContext) {
    printkln!("trap called, but not implemented");
}
