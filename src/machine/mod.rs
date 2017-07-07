use super::{Error, Result};

use std::{self, fmt};
use std::io::Read;
use std::vec::Vec;
use std::process::{Command, Stdio};

/*
 * Represents a CPU setup for a virtual machine
 */
pub struct CpuSetup {
    // Number of global CPUs
    // If this is not specified, one of the three other values must be
    cpus: Option<u8>,

    // Number of sockets
    sockets: Option<u8>,

    // Number of cores per socket
    cores_per_sockets: Option<u8>,

    // Number of thread per core
    threads_per_cores: Option<u8>
}

impl CpuSetup {
    /*
     * Construct a simple CPU setup, give a number of CPUs
     */
    pub fn new(cpus: u8) -> CpuSetup {
        CpuSetup {
            cpus: Some(cpus),
            sockets: None,
            cores_per_sockets: None,
            threads_per_cores: None
        }
    }

    /*
     * Construct a custom CPU setup
     * Must be specified: number of sockets, number of cores per socket
     * and number of threads per core
     */
    pub fn custom(sockets: u8, cores: u8, threads: u8) -> CpuSetup {
        CpuSetup {
            cpus: None,
            sockets: Some(sockets),
            cores_per_sockets: Some(cores),
            threads_per_cores: Some(threads)
        }
    }
}

/*
 * Represents a memory size and layout to be used in a machine
 */
pub struct MemorySetup {
    // Machine's startup RAM size (MiB)
    size: usize,

    // Number of hotpluggable memory slots
    // Only specified if memory hotplug is desired
    slots: Option<u8>,

    // Maximum amout of memory that the machine will be able to handle
    // Only specified if memory hotplug is desired
    maxmem: Option<usize>
}

impl MemorySetup {
    /*
     * Construct a basic memory layout with the specified amout of stratup RAM in MiB
     * Memory hotplug will not be available
     */
    pub fn new(size: usize) -> MemorySetup {
        MemorySetup {
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
    pub fn hotpluggable(size: usize, slots: u8, maxmem: usize) -> MemorySetup {
        MemorySetup {
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
 * Represents a TAP network interface
 */
pub struct TapInterface {
    /*
     * Name/ID of the TAP interface
     * It will be shown by that name on the host
     */
    name: String,

    /*
     * Optional custom MAC address
     */
    custom_mac: Option<String>
}

impl TapInterface {
    /*
     * Construct a new TAP interface with a random MAC address
     */
    pub fn new<S: Into<String>>(name: S) -> TapInterface {
        TapInterface {
            name: name.into(),
            custom_mac: None
        }
    }

    /*
     * Construct a new TAP interface with a random MAC address
     */
    pub fn with_mac_addr<S: Into<String>>(name: S, mac: S) -> TapInterface {
        TapInterface {
            name: name.into(),
            custom_mac: Some(mac.into())
        }
    }
}

/*
 * Representation of a QEMU virtual machine
 */
pub struct Machine {
    kvm: bool,

    cpu: CpuSetup,
    mem: MemorySetup,

    drives: Vec<Drive>,
    interfaces: Vec<TapInterface>
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
    pub fn new(cpu: CpuSetup, mem: MemorySetup, use_kvm: bool) -> Machine {
        Machine {
            kvm: use_kvm,
            cpu: cpu,
            mem: mem,
            drives: Vec::new(),
            interfaces: Vec::new()
        }
    }

    /*
     * Attach a drive to the virtual machine
     */
    pub fn add_drive(&mut self, drive: Drive) {
        self.drives.push(drive);
    }

    /*
     * Attach a TAP network interface to the virtual machine
     */
    pub fn add_interface(&mut self, iface: TapInterface) {
        self.interfaces.push(iface);
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

        // CPU setup
        if let Some(cpus) = self.cpu.cpus {
            if cpus > 0 {
                cmd.args(&["-smp", cpus.to_string().as_ref()]);
            }
            else {
                return Err(Error::InvalidArgument("Invalid number of CPUs (must be greater than 0)".to_owned()));
            }
        }
        // If custom CPU setup
        else if self.cpu.sockets.is_some() && self.cpu.cores_per_sockets.is_some() && self.cpu.threads_per_cores.is_some() {
            let sockets = self.cpu.sockets.unwrap();
            let cores = self.cpu.cores_per_sockets.unwrap();
            let threads = self.cpu.threads_per_cores.unwrap();

            // Manually specify the number of sockets, cores and threads
            if sockets > 0 && cores > 0 && threads > 0 {
                cmd.args(&[
                    "-smp",
                    format!("cores={},threads={},sockets={}", cores, threads, sockets).as_ref()
                ]);
            }
            else {
                return Err(Error::InvalidArgument("Invalid CPU parameters (sockets, cores, threads) (must be greater than 0)".to_owned()));
            }
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

        // Setup network interfaces
        for iface in &self.interfaces {
            let mut dev = format!("virtio-net,netdev={}", iface.name);

            // If we specified a custom MAC address
            if let Some(ref mac) = iface.custom_mac {
                if mac.len() > 0 {
                    dev = format!("{},mac={}", dev, mac);
                }
                else {
                    return Err(Error::InvalidArgument("Invalid custom MAC address".to_owned()));
                }
            }

            // Add the correct arguments to the command line
            // First the netdev, then de virtio device (cf man qemu)
            cmd.args(&[
                "-netdev", format!("tap,id={},ifname={}", iface.name, iface.name).as_ref(),
                "-device", dev.as_ref()
            ]);
        }

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
