#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let mut lock = $crate::writer::buffer::WRITER.lock();
        let writer = lock.as_mut().unwrap();
        writer.write_fmt(format_args!($($arg)*)).expect("Could not write the message");
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!($($arg)*);
        $crate::print!("\n");
    }};
}
