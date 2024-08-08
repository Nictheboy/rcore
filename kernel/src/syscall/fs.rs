use crate::print;

pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    if fd == 1 {
        print!("{}", core::str::from_utf8(buf).unwrap());
        0
    } else {
        -1
    }
}
