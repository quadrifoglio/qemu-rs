//! Basic usage example for qemu-rs.
//! It just starts QEMU without any operating system.

extern crate qemu;

use qemu::machine::Processors;

fn main() {
    let builder = qemu::Builder::new("qemu-system-x86_64").unwrap()
        .set(Processors::with(Some(1), Some(2), None).unwrap().set_max_cpus(255));

    let emulator = builder.start().unwrap();
}
