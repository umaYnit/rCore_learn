#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::yield_;

const WIDTH: usize = 10;
const HEIGHT: usize = 2;

#[no_mangle]
fn main() -> i32 {
    for i in 0..HEIGHT {
        println!("{:B<1$}", "", WIDTH);
        println!("[{}/{}]", i + 1, HEIGHT);
        yield_();
    }
    println!("Test write_b OK!");
    0
}