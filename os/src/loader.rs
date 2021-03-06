use crate::config::*;
use crate::task::TaskContext;
use crate::trap::TrapContext;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

const EMPTY_KERNEL_STACK: KernelStack = KernelStack { data: [0; KERNEL_STACK_SIZE] };
const EMPTY_USER_STACK: UserStack = UserStack { data: [0; USER_STACK_SIZE] };

static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [EMPTY_KERNEL_STACK; MAX_APP_NUM];
static USER_STACK: [UserStack; MAX_APP_NUM] = [EMPTY_USER_STACK; MAX_APP_NUM];

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, trap_cx: TrapContext, task_cx: TaskContext) -> &'static mut TaskContext {
        unsafe {
            let trap_cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
            *trap_cx_ptr = trap_cx;
            let task_cx_ptr = (trap_cx_ptr as usize - core::mem::size_of::<TaskContext>()) as *mut TaskContext;
            *task_cx_ptr = task_cx;
            task_cx_ptr.as_mut().unwrap()
        }
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

pub fn get_num_app() -> usize {
    extern "C" { fn _num_app(); }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

pub fn load_apps() {
    extern "C" { fn _num_app(); }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();

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

pub fn init_app_cx(app_id: usize) -> &'static TaskContext {
    KERNEL_STACK[app_id].push_context(
        TrapContext::app_init_context(get_base_i(app_id), USER_STACK[app_id].get_sp()),
        TaskContext::goto_restore(),
    )
}