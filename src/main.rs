#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;

global_asm!(include_str!("entry.asm"));

const SBI_SHUTDOWN: usize = 8;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x17}" (id)
            : "memory"
            : "volatile"
        );
    }
    ret
}

pub fn shutdown() -> ! {
    syscall(SBI_SHUTDOWN, [0, 0, 0]);
    panic!("It should shutdown!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    println!("clear bss");
    (sbss as usize..ebss as usize).for_each(|a| { unsafe { (a as *mut u8).write_volatile(0) } });
}

#[no_mangle]
pub fn rust_main() {
    println!("hello world");
    shutdown();
}