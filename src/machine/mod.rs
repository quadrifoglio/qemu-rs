use std::fmt;
use std::vec::Vec;

/*
 * Represents a memory size and layout to be used in a machine
 */
pub struct Memory {
    // Machine's startup RAM size (bytes)
    size: usize,

    // Number of hotpluggable memory slots
    // Only specified if memory hotplug is desired
    slots: Option<u8>,

    // Maximum amout of memory that the machine will be able to handle
    // Only specified if memory hotplug is desired
    maxmem: Option<usize>
}

impl Memory {
    /*
     * Construct a basic memory layout with the specified amout of stratup RAM in bytes
     * Memory hotplug will not be available
     */
    pub fn new(size: usize) -> Memory {
        Memory {
            size: size,
            slots: None,
            maxmem: None
        }
    }

    /*
     * Construct a memory layout which supports memory hotplug with the specified amout
     * of startup RAM (bytes), the specifed number of hotpluggable memory slots, and
     * the maximum amout of RAM (bytes) that the guest will be able to handle (bytes)
     */
    pub fn hotpluggable(size: usize, slots: u8, maxmem: usize) -> Memory {
        Memory {
            size: size,
            slots: Some(slots),
            maxmem: Some(size)
        }
    }
}

/*
 * List of supported drive media
 */
#[derive(Debug)]
pub enum DriveMedia {
    CDRom,
    Disk
}

/*
 * Display trait implementation
 * Allows the media types to be displayed as strings
 * QEMU expects lowercase
 */
impl fmt::Display for DriveMedia {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

/*
 * Representation of a drive that can be used in a machine
 * Drives are based on an image file
 */
pub struct Drive {
    media: DriveMedia,
    file: String,
}

impl Drive {
    /*
     * Construct a new drive with the specified media type
     * and image file path
     */
    pub fn new<S: Into<String>>(media: DriveMedia, file: S) -> Drive {
        Drive {
            media: media,
            file: file.into()
        }
    }
}

/*
 * Representation of a QEMU virtual machine
 */
pub struct Machine {
    kvm: bool,
    mem: Memory,
    drives: Vec<Drive>
}

impl Machine {
    /*
     * Construct a new virtual machine with the specified memory setup,
     * with the option to use KVM-based hardware acceleration
     */
    pub fn new(mem: Memory, use_kvm: bool) -> Machine {
        Machine {
            kvm: use_kvm,
            mem: mem,
            drives: Vec::new()
        }
    }

    /*
     * Attach a drive to the virtual machine
     */
    pub fn add_drive(&mut self, drive: Drive) {
        self.drives.push(drive);
    }
}

/*
 * Unit tests
 */
#[cfg(test)]
mod tests;
