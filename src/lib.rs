//! # qemu-rs
//! QEMU as a Rust library.

#[macro_use]
extern crate failure;

pub mod error;

use std::env;
use std::path::Path;
use std::ffi::OsString;
use std::process::{Command, Child};

use error::Result;

/// Object used to initialize a new QEMU instance with the specified parameters.
pub struct Builder {
    executable: OsString,
    params: Vec<Box<Parameter>>,
}

impl Builder {
    /// Create a new Builder with the specified QEMU executable.
    /// The executable can be a path to QEMU, or just the name of the program if it can be found
    /// using the system's PATH.
    pub fn new<S: Into<String>>(executable: S) -> Result<Builder> {
        let exec = executable.into();

        let exec_path = match Path::new(&exec).exists() {
            false => {
                let path = env::var_os("PATH").and_then(|paths| {
                    for path in env::split_paths(&paths) {
                        let path = path.join(&exec);

                        if path.is_file() {
                            return Some(path);
                        }
                    }

                    None
                });

                if let Some(path) = path {
                    path.into_os_string()
                } else {
                    return Err(error::InitError::ExecutableNotFound{exec: exec}.into());
                }
            },

            true => exec.into(),
        };

        Ok(Builder {
            executable: exec_path,
            params: Vec::new(),
        })
    }

    /// Start the QEMU emulator. Immediatly returns the control to the control to the caller, does
    /// not wait on the spawned child process.
    pub fn start(self) -> Result<Instance> {
        let mut command = Command::new(self.executable);
        let child = command.spawn()?;

        Ok(Instance {
            process: child,
        })
    }
}

/// Represents an running QEMU instance.
pub struct Instance {
    process: Child,
}

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
