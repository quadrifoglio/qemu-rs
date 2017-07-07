use super::{Error, Result};

use std::{self, fmt};
use std::io::Read;
use std::vec::Vec;
use std::process::{Command, Stdio};

/*
 * Represents a memory size and layout to be used in a machine
 */
pub struct Memory {
    // Machine's startup RAM size (MiB)
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
     * Construct a basic memory layout with the specified amout of stratup RAM in MiB
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
     * of startup RAM (MiB), the specifed number of hotpluggable memory slots, and
     * the maximum amout of RAM (MiB) that the guest will be able to handle (MiB)
     */
    pub fn hotpluggable(size: usize, slots: u8, maxmem: usize) -> Memory {
        Memory {
            size: size,
            slots: Some(slots),
            maxmem: Some(maxmem)
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

/*
 * Representation of a machine's current status
 */
pub struct MachineStatus {
    /*
     * True if the machine is currently running
     */
    pub running: bool
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

    /*
     * Start the virtual machine
     */
    pub fn start(&self) -> Result<MachineStatus> {
        // Prepare the command that will be run
        let mut cmd = Command::new("qemu-system-x86_64");

        // Capture stderr, in order to be able to read it in case of a QEMU error
        cmd.stderr(Stdio::piped());

        // Only enable KVM hardware acceleration if requested
        if self.kvm {
            cmd.arg("-enable-kvm");
        }

        // Memory argument
        // Syntax: <size>,slots<slots>,maxmem=<maxmem>
        let mut mem_arg = String::new();

        if self.mem.size > 0 {
            mem_arg += self.mem.size.to_string().as_ref();
        }
        else {
            return Err(Error::InvalidArgument("Invalid memory size (must be greater than 0)".to_owned()));
        }

        // If both slots and maxmem are specified, add them to the memory argument
        if self.mem.slots.is_some() && self.mem.maxmem.is_some() {
            let slots = self.mem.slots.unwrap();
            let maxmem = self.mem.maxmem.unwrap();

            // If slots is not zero
            if slots > 0 {
                mem_arg += format!(",slots={}", slots.to_string()).as_ref();
            }
            else {
                return Err(Error::InvalidArgument("Invalid memory slots number (must be greater than 0)".to_owned()));
            }

            // If maxmem is not zero
            if maxmem > 0 {
                mem_arg += format!(",maxmem={}", maxmem.to_string()).as_ref();
            }
            else {
                return Err(Error::InvalidArgument("Invalid maximum memory amout (must be greater than 0)".to_owned()));
            }
        }

        // Add the memory setup to the command line arguments
        cmd.args(&["-m", mem_arg.as_ref()]);

        // Run the command as a background process
        match cmd.spawn() {
            // If running the command succeeded
            Ok(mut child) => {
                // Sleep for 500ms to give QEMU some time to boot
                std::thread::sleep(std::time::Duration::from_millis(500));

                // Check if QEMU is still running
                match child.try_wait() {
                    // If QEMU is still running after 500ms, consider the startup successful
                    Ok(None) => Ok(MachineStatus {
                        running: true
                    }),

                    // If QEMU exited, return the error
                    Ok(Some(status)) => {
                        // Buffer to store stderr data
                        let mut out = String::new();

                        // Read stderr
                        match child.stderr.unwrap().read_to_string(&mut out) {
                            Ok(_) => Err(Error::Runtime(format!("QEMU exited: {}", out))),
                            _ => Err(Error::Runtime("QEMU exited unexpectedly".to_owned()))
                        }
                    },

                    // If checking the process' status failed
                    Err(err) => Err(Error::Io(err))
                }
            },
            // If it failed
            Err(err) => Err(Error::Io(err))
        }
    }
}

/*
 * Unit tests
 */
#[cfg(test)]
mod tests;
