//! # qemu-rs
//! QEMU as a Rust library.

/// Trait that represent a command line parameter that can be passed to QEMU.
/// Pair of (parameter_name, parameter_value).
/// Example: ('name', 'My VM').
pub trait Parameter {
    /// Returns the name of the command line parameter.
    /// Examples: 'display', 'smp', 'm'...
    fn name(&self) -> &str;

    /// Returns the value for a command line parameter, if any.
    /// Examples for the 'display' parameter name: 'sdl', 'curses', 'none'...
    fn value(&self) -> Option<&str>;

    /// Take ownership of the parameter. Returns its name and value.
    /// Consumes `self`.
    fn take(self) -> (String, Option<String>);
}

impl Parameter for &'static str {
    fn name(&self) -> &str {
        self
    }

    fn value(&self) -> Option<&str> {
        None
    }

    fn take(self) -> (String, Option<String>) {
        (self.into(), None)
    }
}

impl Parameter for String {
    fn name(&self) -> &str {
        self.as_ref()
    }

    fn value(&self) -> Option<&str> {
        None
    }

    fn take(self) -> (String, Option<String>) {
        (self, None)
    }
}

impl<S: AsRef<str> + Into<String>> Parameter for (S, S) {
    fn name(&self) -> &str {
        self.0.as_ref()
    }

    fn value(&self) -> Option<&str> {
        Some(self.1.as_ref())
    }

    fn take(self) -> (String, Option<String>) {
        (self.0.into(), Some(self.1.into()))
    }
}

#[cfg(test)]
mod tests;
