use palloc::SpinPalloc;

use crate::println;
use core::panic::PanicInfo;

#[global_allocator]
pub static mut ALLOCATOR: SpinPalloc = SpinPalloc::empty();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("Panicked at {}:{}!", location.file(), location.line(),);
    } else {
        crate::println!("Panicked!");
    }
    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::SystemFailure);
    loop {}
}
