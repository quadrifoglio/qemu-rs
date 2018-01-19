//! Basic usage example for qemu-rs.
//! It just starts QEMU without any operating system.

extern crate qemu;

use qemu::machine::{Processors, Memory};
use qemu::display::{Display, Vnc};

fn main() {
    let builder = qemu::Builder::new("qemu-system-x86_64").unwrap()
        .set(Processors::new(1).set_max_cpus(255))
        .set(Memory::new(128))
        .set(Display::Sdl);

    let emulator = builder.start().unwrap();
}
