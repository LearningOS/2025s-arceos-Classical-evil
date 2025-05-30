//! Standard library macros

/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// [`println!`]: crate::println
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!($($arg)*));
    }
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        // $crate::io::__print_impl(format_args!(r"\33[30m"), format_args!("{}\n"), format_args!($($arg)*), format_args!(r"\33[0m"));
        $crate::io::__print_impl(format_args!("{}{}{}\n", 
                format_args!("\x1b[31m"), format_args!($($arg)*), format_args!("\x1b[0m")));
    }
}
