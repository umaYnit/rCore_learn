use crate::batch::{APP_BASE_ADDRESS, APP_SIZE_LIMIT, USER_STACK, USER_STACK_SIZE};

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let stack_end = USER_STACK.get_sp();
            let stack_start = stack_end - USER_STACK_SIZE;

            let start = buf as usize;

            if arr_not_in_scope!(start,len,
            [stack_start,stack_start],
            [APP_BASE_ADDRESS,APP_BASE_ADDRESS + APP_SIZE_LIMIT]) {
                println!("[kernel] Invalid write message, start = {:#x} len = {}", start, len);
                return -1;
            }

            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = unsafe { core::str::from_utf8_unchecked(slice) };
            print!("{}", str);
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}