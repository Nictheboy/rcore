use sbi_rt::Physical;

pub fn write_str(s: &str) {
    let bytes = s.as_bytes();
    let num_bytes = bytes.len();
    let ptr = bytes.as_ptr() as usize;
    let phys_addr_lo = ptr & 0xFFFF_FFFF;
    let phys_addr_hi = ptr >> 32;
    sbi_rt::console_write(Physical::new(num_bytes, phys_addr_lo, phys_addr_hi));
}
