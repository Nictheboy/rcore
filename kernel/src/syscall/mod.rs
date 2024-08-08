pub mod fs;
pub mod process;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

pub fn syscall_handler(id: usize, args: [usize; 3]) -> isize {
    match id {
        SYSCALL_WRITE => fs::sys_write(args[0], unsafe {
            core::slice::from_raw_parts(args[1] as *const u8, args[2])
        }),
        SYSCALL_EXIT => process::sys_exit(args[0]),
        _ => {
            crate::printkln!("Unknown syscall: {}", id);
            -1
        }
    }
}
