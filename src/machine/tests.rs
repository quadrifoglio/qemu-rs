use image::{Image, Format};
use super::{Drive, DriveMedia, Memory, Machine};

#[test]
fn drive_media_representations() {
    assert_eq!(DriveMedia::CDRom.to_string().as_str(), "cdrom");
    assert_eq!(DriveMedia::Disk.to_string().as_str(), "disk");
}

#[test]
fn start_memory_basic_valid() {
    let machine = Machine::new(Memory::new(512), false);
    machine.start().unwrap();
}

#[test]
#[should_panic]
fn start_memory_basic_invalid() {
    let machine = Machine::new(Memory::new(0), false);
    machine.start().unwrap();
}

#[test]
fn start_memory_hotpluggable_valid() {
    let machine = Machine::new(Memory::hotpluggable(1024, 3, 4096), false);
    machine.start().unwrap();
}

#[test]
#[should_panic]
fn start_memory_hotpluggable_invalid_1() {
    let machine = Machine::new(Memory::hotpluggable(1024, 0, 4096), false);
    machine.start().unwrap();
}

#[test]
#[should_panic]
fn start_memory_hotpluggable_invalid_2() {
    let machine = Machine::new(Memory::hotpluggable(1024, 3, 0), false);
    machine.start().unwrap();
}
