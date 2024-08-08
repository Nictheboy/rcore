use alloc::string::String;

pub enum Fault {
    Unknown(String),    // Unknown fault with a description
    MemoryFault(usize), // Memory fault with address
    IllegalInstruction,
    Breakpoint,
}

pub type SyscallHandler = fn(id: usize, args: [usize; 3]) -> isize;
pub type FaultHandler = fn(Fault);

pub trait Arch {
    fn memory_boot_bss() -> &'static mut [u8];
    fn memory_boot_heap() -> &'static mut [u8];
    fn context_switch_to_user(user_pc: usize, user_stack: usize) -> !;
    fn trap_init(syscall_handler: SyscallHandler, fault_handler: FaultHandler);
    fn debug_print(content: &str);
}

// Current Arch:
mod riscv64;
pub type CurrentArch = riscv64::RiscV64;
