//! Error handling functionality.

use failure;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

#[derive(Debug, Fail)]
pub enum InitError {
    #[fail(display = "could not find QEMU executable: {}", exec)]
    ExecutableNotFound {
        exec: String,
    },

    #[fail(display = "invalid QEMU configuration: {}", msg)]
    InvalidConfig {
        msg: String,
    },
}
