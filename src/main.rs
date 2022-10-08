#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
}
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a|{
        unsafe{(a as *mut u8).write_volatile(0)}
    });
}


#[macro_use]
mod console;


mod long_items;
mod sbi;
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
//println!("hello world {}!",i);
