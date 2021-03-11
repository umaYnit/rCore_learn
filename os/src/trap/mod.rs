use riscv::register::{mtvec::TrapMode, scause::{self, Exception, Trap}, sie, stval, stvec};

use crate::syscall::sys_exit;
use crate::syscall::syscall;
use crate::task::exit_current_and_run_next;
pub use crate::trap::context::TrapContext;

mod context;

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" { fn __alltraps(); }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, core dumped.");
            sys_exit(-1)
            // run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, core dumped.");
            sys_exit(-1);
            // run_next_app();
        }
        _ => panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval)
    }
    cx
}