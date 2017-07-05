use super::{Error, Result};

use std::fmt;
use std::process::Command;

/*
 * List of all image formats supported by QEMU
 */
#[derive(Debug)]
pub enum Format {
    Raw,
    QCow,
    QCow2,
    Vmdk,
    Vdi,
    Vhdx,
    Vpc
}

/*
 * Display trait implementation
 * Allows the formats to be displayed as strings
 * QEMU expects formats in lowercase
 */
impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

/*
 * Representation of a QEMU image
 */
pub struct Image {
    /*
     * Path of the image
     */
    pub path: String,

    /*
     * Format of the QEMU image (raw, qcow2...)
     */
    pub format: Format,

    /*
     * Size of the image in bytes
     */
    pub size: usize
}

impl Image {
    /*
     * Image creation: create a new image representation
     */
    pub fn new(path: &str, format: Format, size: usize) -> Image {
        Image {
            path: path.to_owned(),
            format: format,
            size: size
        }
    }

    /*
     * Image creation: actually write the image to disk
     */
    pub fn write(&self) -> Result<()> {
        // Prepare and execute the creation command
        // Syntax: `qemu-img create -f <format> <path> <size in bytes>`
        let out = Command::new("qemu-img")
            .arg("create")
            .arg("-f")
            .arg(self.format.to_string())
            .arg(self.path.as_str())
            .arg(self.size.to_string())
            .output();

        match out {
            // If the command executed
            Ok(out) => match out.status.success() {
                // If the return status is 0 (success), we are done. Exit normally
                true => Ok(()),
                // If the command did not run successfully, return the error message to the caller
                false => Err(Error::Other(String::from_utf8(out.stdout).expect("Invalid UTF-8 returned by qemu-img")))
            },
            // If the command failed to run
            Err(err) => Err(Error::Io(err))
        }
    }
}

/*
 * Tests module
 */
#[cfg(test)]
mod tests;
