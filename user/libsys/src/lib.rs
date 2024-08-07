#![no_std]

use core::arch::asm;
use core::unreachable;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            in("x17") id,
            in("x10") args[0],
            in("x11") args[1],
            in("x12") args[2],
            lateout("x10") ret,
        );
    }
    ret
}

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buf.as_ptr() as usize, buf.len()])
}

pub fn sys_exit(code: usize) -> ! {
    syscall(SYSCALL_EXIT, [code, 0, 0]);
    unreachable!()
}
