use core::fmt;

struct DebugWritePutChar;

impl fmt::Write for DebugWritePutChar {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &c in s.as_bytes() {
            sel4_sys::seL4_DebugPutChar(c)
        }
        Ok(())
    }
}

pub fn _debug_print_putchar(args: fmt::Arguments) {
    fmt::write(&mut DebugWritePutChar {}, args).unwrap();
}

macro_rules! debug_print {
    ($($arg:tt)*) => ($crate::debug::_debug_print_putchar(format_args!($($arg)*)));
}

macro_rules! debug_println {
               () => ($crate::debug::debug_print!("\n"));
    ($($arg:tt)*) => ($crate::debug::debug_print!("{}\n", format_args!($($arg)*)));
}

pub(crate) use debug_print;
pub(crate) use debug_println;
