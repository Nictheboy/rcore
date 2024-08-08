use crate::{
    arch::{Fault, FaultHandler, SyscallHandler},
    printkln,
};
use alloc::format;
use core::arch::asm;

static mut SYSCALL_HANDLER: SyscallHandler = default_syscall_handler;
static mut FAULT_HANDLER: FaultHandler = default_fault_handler;

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: usize,
    pub sepc: usize,
}

pub unsafe fn trap_init(syscall_handler: SyscallHandler, fault_handler: FaultHandler) {
    extern "C" {
        fn _trap_handler();
    }
    SYSCALL_HANDLER = syscall_handler;
    FAULT_HANDLER = fault_handler;
    asm!(
        "csrw stvec, {_trap_handler}",
        _trap_handler = in(reg) _trap_handler,
    );
}

fn default_syscall_handler(_id: usize, _args: [usize; 3]) -> isize {
    printkln!("Syscall handler not set!");
    -1
}

fn default_fault_handler(_fault: Fault) {
    printkln!("Fault handler not set!");
}

#[no_mangle]
pub unsafe fn trap_handler(trap_context: &mut TrapContext) {
    let scause: usize;
    let stval: usize;
    asm!(
        "csrr {0}, scause",
        "csrr {1}, stval",
        out(reg) scause,
        out(reg) stval,
    );
    if scause == 8 {
        unsafe {
            let ret = SYSCALL_HANDLER(
                trap_context.x[17],
                [trap_context.x[10], trap_context.x[11], trap_context.x[12]],
            );
            trap_context.x[10] = ret as usize;
        }
    } else {
        let fault = match scause {
            2 => Fault::IllegalInstruction,
            3 => Fault::Breakpoint,
            5 => Fault::MemoryFault(stval),
            7 => Fault::MemoryFault(stval),
            12 => Fault::MemoryFault(stval),
            13 => Fault::MemoryFault(stval),
            15 => Fault::MemoryFault(stval),
            _ => Fault::Unknown(format!("Unknown scause: {:#x}", scause)),
        };
        unsafe {
            FAULT_HANDLER(fault);
        }
    }
    trap_context.sepc = next_instruction_address(trap_context.sepc);
}

fn next_instruction_address(pc: usize) -> usize {
    if pc & 0b11 == 0b11 {
        pc + 2
    } else {
        pc + 4
    }
}
