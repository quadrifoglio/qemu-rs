//! QEMU machine options.

use error::{InitError, Result};
use std::collections::HashMap;

/// Represents the CPU settings of the emulated SMP system.
pub struct Processors {
    ncpus: Option<u8>,
    cores: Option<u8>,
    threads: Option<u8>,
    sockets: Option<u8>,
    maxcpus: Option<u8>,
}

impl Processors {
    /// Define a system with `n` CPUs.
    pub fn new(n: u8) -> Processors {
        Processors {
            ncpus: Some(n),
            cores: None,
            threads: None,
            sockets: None,
            maxcpus: None,
        }
    }

    /// Define a system with the specified number of CPU cores, threads, and sockets.
    /// Missing values will be computed. The only case this function will return an error is if
    /// none of the parameters have a value.
    pub fn with(cores: Option<u8>, threads: Option<u8>, sockets: Option<u8>) -> Result<Processors> {
        if cores.is_none() && threads.is_none() && sockets.is_none() {
            return Err(InitError::InvalidConfig{msg: String::from("cpu cores, threads, or sockets must be defined")}.into());
        }

        Ok(Processors {
            ncpus: None,
            cores: cores,
            threads: threads,
            sockets: sockets,
            maxcpus: None,
        })
    }

    /// Set the maximum number of hotpluggable CPUs.
    pub fn set_max_cpus(mut self, n: u8) -> Self {
        self.maxcpus = Some(n);
        self
    }
}

impl super::IntoArguments for Processors {
    fn into_arguments(self) -> Vec<String> {
        let mut opts = HashMap::new();

        if let Some(ncpus) = self.ncpus {
            opts.insert(String::from("cpus"), format!("{}", ncpus));
        } else {
            if let Some(cores) = self.cores {
                opts.insert(String::from("cores"), format!("{}", cores));
            }
            if let Some(threads) = self.threads {
                opts.insert(String::from("threads"), format!("{}", threads));
            }
            if let Some(sockets) = self.sockets {
                opts.insert(String::from("sockets"), format!("{}", sockets));
            }
        }

        if let Some(maxcpus) = self.maxcpus {
            opts.insert(String::from("maxcpus"), maxcpus.to_string());
        }

        let mut settings = opts.into_iter()
            .map(|(opt, val)| format!("{}={},", opt, val))
            .fold(String::new(), |mut a, b| { a.push_str(&b); a });

        // Remove trailing coma.
        settings.pop();

        vec![String::from("-smp"), settings]
    }
}

/// Represents RAM settings.
pub struct Memory {
    size: u64,
    slots: Option<u8>,
    maxmem: Option<u64>,
}

impl Memory {
    /// Construct a new Memory object with the specified amout of RAM in MiB. Hotpluggable memory
    /// will not be available.
    pub fn new(size: u64) -> Memory {
        Memory {
            size: size,
            slots: None,
            maxmem: None,
        }
    }

    /// Construct a new Memory object with the specified amount of RAM in MiB, and a defined number
    /// of hotpluggable memory slots and amount in MiB. Not that `maxmem` mut be aligned with the
    /// page size.
    pub fn with(size: u64, slots: u8, maxmem: u64) -> Memory {
        Memory {
            size: size,
            slots: Some(slots),
            maxmem: Some(maxmem),
        }
    }
}

impl super::IntoArguments for Memory {
    fn into_arguments(self) -> Vec<String> {
        let mut settings = format!("size={}", self.size);

        if self.slots.is_some() && self.maxmem.is_some() {
            settings.push_str(format!(",slots={}", self.slots.unwrap()).as_str());
            settings.push_str(format!(",maxmem={}", self.maxmem.unwrap()).as_str());
        }

        vec![String::from("-m"), settings]
    }
}
