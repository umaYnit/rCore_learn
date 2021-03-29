use core::fmt::{self, Write};

use log::{debug, Level, LevelFilter, Log, Metadata, Record};
use spin::Mutex;

use crate::sbi::console_putchar;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes().map(|c| c as usize).for_each(console_putchar);
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    static LOCK: Mutex<()> = Mutex::new(());
    let _guard = LOCK.lock();
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}



pub fn init(hart_id: usize) {
    use core::sync::atomic::{AtomicBool, Ordering};
    // 只初始化一次
    static INITED: AtomicBool = AtomicBool::new(false);
    if INITED.compare_exchange(false,
                               true,
                               Ordering::Acquire,
                               Ordering::Relaxed)
        .is_err() {
        return;
    }

    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some(data) => {
            match data {
                "error" => LevelFilter::Error,
                "warn" => LevelFilter::Warn,
                "info" => LevelFilter::Info,
                "debug" => LevelFilter::Debug,
                "trace" => LevelFilter::Trace,
                _ => LevelFilter::Off,
            }
        }
        _ => LevelFilter::Off,
    });

    debug!("[{}] init color log", hart_id);
}


pub struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        println!("\x1b[{}m[{}]\t{}\x1b[0m",
                 level_to_color_code(record.level()),
                 record.level(),
                 record.args());
    }
    fn flush(&self) {}
}


fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}

