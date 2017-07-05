use std::fmt;

/*
 * List of all image formats supported by QEMU
 */
#[derive(Debug)]
pub enum Format {
    Raw,
    Cloop,
    Cow,
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

/*
 * Tests module
 */
#[cfg(test)]
mod tests;
