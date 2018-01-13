//! # qemu-rs
//! QEMU as a Rust library.

#[macro_use]
extern crate failure;

pub mod error;
pub mod machine;

use std::env;
use std::path::Path;
use std::ffi::OsString;
use std::process::{Command, Child};

use error::Result;

/// Object used to initialize a new QEMU instance with the specified parameters.
pub struct Builder {
    executable: OsString,
    params: Vec<String>,
}

impl Builder {
    /// Create a new Builder with the specified QEMU executable.
    /// That executable can either be the path to a QEMU binary, or just its name, in which case it
    /// will be resolved using the system's PATH environment variable.
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

    /// Use the behavior defined in the specified object, and pass it as QEMU emulator options.
    pub fn set<A: IntoArguments>(mut self, a: A) -> Self {
        self.params.extend(a.into_arguments());
        self
    }

    /// Start the QEMU emulator. Immediatly returns the control to the control to the caller, does
    /// not wait on the spawned child process.
    pub fn start(self) -> Result<Instance> {
        let mut command = Command::new(self.executable);

        for param in self.params {
            command.arg(param);
        }

        Ok(Instance {
            process: command.spawn()?,
        })
    }
}

/// Represents an running QEMU instance.
pub struct Instance {
    process: Child,
}

/// Trait implemented for every object that represent some kind of option of the QEMU emulator.
pub trait IntoArguments {
    /// Must return the list of command line arguments that will be passed to QEMU.
    fn into_arguments(self) -> Vec<String>;
}

#[cfg(test)]
mod tests;
