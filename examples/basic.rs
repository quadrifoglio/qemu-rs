//! Basic usage example for qemu-rs.
//! It just starts QEMU without any operating system.

extern crate qemu;

fn main() {
    let builder = qemu::Builder::new("qemu-system-x86_64").unwrap();
    let emulator = builder.start().unwrap();
}
