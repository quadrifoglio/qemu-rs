/*
 * Custom Error type for this library
 */
#[derive(Debug)]
pub enum Error {
    /*
     * Input/Output error
     */
    Io(std::io::Error),

    /*
     * An invalid argument value was specified
     */
    InvalidArgument(String),

    /*
     * QEMU unexpectedly exited
     * Contains the outputed stdout data
     */
    Runtime(String),

    /*
     * Other type of error
     */
    Other(String),

    /*
     * Unknown error
     */
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
