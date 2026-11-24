pub fn print_str(s: &str) {
    unsafe {
        libc::write(1, s.as_ptr() as _, s.len());
        libc::write(1, "\n".as_ptr() as _, 1);
    }
}

/// NAK demo: `core::fmt` output to stdout, to print the error that ended the session.
pub struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe { libc::write(1, s.as_ptr() as _, s.len()) };
        Ok(())
    }
}
