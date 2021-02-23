use crate::config::*;
use crate::trap::TrapContext;
use lazy_static::*;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack { data: [0; KERNEL_STACK_SIZE] };
static USER_STACK: UserStack = UserStack { data: [0; USER_STACK_SIZE] };

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
            cx_ptr.as_mut().unwrap()
        }
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

static mut CURRENT_APP_ID: usize = 0;
lazy_static! {
    static ref NUM_APP: usize = {
        extern "C" { fn _num_app(); }
        let num_app_ptr = _num_app as usize as *const usize;
        unsafe { num_app_ptr.read_volatile() }
    };
}

fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}


pub fn load_apps() {
    extern "C" { fn _num_app(); }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = unsafe { num_app_ptr.read_volatile() };

    let app_start = unsafe {
        core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1)
    };

    // clear icache
    unsafe { llvm_asm!("fence.i" :::: "volatile"); }
    // load apps
    for i in 0..num_app {
        let base_i = get_base_i(i);

        // clear app area
        (base_i..).take(APP_SIZE_LIMIT).for_each(|addr| unsafe {
            (addr as *mut u8).write_volatile(0)
        });

        let len = app_start[i + 1] - app_start[i];
        unsafe { core::ptr::copy_nonoverlapping(app_start[i] as *const u8, base_i as *mut u8, len); }
    }
}

pub fn init() {
    load_apps();
}


pub fn run_next_app() -> ! {
    extern "C" { fn __restore(cx_addr: usize); }
    let current_app_addr = get_base_i(unsafe { CURRENT_APP_ID });
    println!("{:#X}", current_app_addr);
    unsafe {
        CURRENT_APP_ID += 1;
        if CURRENT_APP_ID > *NUM_APP {
            panic!("All applications completed!");
        }
    }
    unsafe {
        __restore(KERNEL_STACK.push_context(
            TrapContext::app_init_context(current_app_addr, USER_STACK.get_sp())
        ) as *const _ as usize);
    }
    panic!("Unreachable in batch::run_current_app!");
}