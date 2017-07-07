/*
 * Custom Error type for this library
 */
pub enum Error {
    Io(std::io::Error),
    Other(String),
    Unknown
}

/*
 * Custom Result type for this library
 */
pub type Result<T> = std::result::Result<T, Error>;

/*
 * QEMU image manipulation
 */
pub mod image;

/*
 * QEMU virtual machine management
 */
pub mod machine;
