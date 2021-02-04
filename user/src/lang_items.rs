use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("custom Panicked at {}:{} {}", location.file(), location.line(), info.message().unwrap());
    } else {
        println!("custom Panicked: {}", info.message().unwrap());
    }
    loop {}
}