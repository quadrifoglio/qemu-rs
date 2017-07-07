use super::{Drive, DriveMedia, CpuSetup, MemorySetup, TapInterface, Machine};
use image::{Image, Format};

use std::path::Path;

#[test]
fn drive_media_representations() {
    assert_eq!(DriveMedia::CDRom.to_string().as_str(), "cdrom");
    assert_eq!(DriveMedia::Disk.to_string().as_str(), "disk");
}

#[test]
fn start_memory_basic_valid() {
    let machine = Machine::new(CpuSetup::new(1), MemorySetup::new(512), false);
    machine.start().unwrap();
}

#[test]
#[should_panic]
fn start_memory_basic_invalid() {
    let machine = Machine::new(CpuSetup::new(1), MemorySetup::new(0), false);
    machine.start().unwrap();
}

#[test]
fn start_memory_basic_kvm_valid() {
    if Path::new("/dev/kvm").exists() {
        let machine = Machine::new(CpuSetup::new(1), MemorySetup::new(512), true);
        machine.start().unwrap();
    }
}

#[test]
fn start_memory_hotpluggable_valid() {
    let machine = Machine::new(CpuSetup::new(1), MemorySetup::hotpluggable(1024, 3, 4096), false);
    machine.start().unwrap();
}

#[test]
#[should_panic]
fn start_memory_hotpluggable_invalid_1() {
    let machine = Machine::new(CpuSetup::new(1), MemorySetup::hotpluggable(1024, 0, 4096), false);
    machine.start().unwrap();
}

#[test]
#[should_panic]
fn start_memory_hotpluggable_invalid_2() {
    let machine = Machine::new(CpuSetup::new(1), MemorySetup::hotpluggable(1024, 3, 0), false);
    machine.start().unwrap();
}

#[test]
fn start_multiple_cpus_basic_valid() {
    let machine = Machine::new(CpuSetup::new(2), MemorySetup::new(512), false);
    machine.start().unwrap();
}

#[test]
fn start_multiple_cpus_basic_invalid() {
    let machine = Machine::new(CpuSetup::new(0), MemorySetup::new(512), false);
    machine.start().unwrap();
}

#[test]
fn start_multiple_cpus_custom_valid() {
    let machine = Machine::new(CpuSetup::custom(1, 4, 2), MemorySetup::new(512), false);
    machine.start().unwrap();
}

#[test]
fn start_multiple_cpus_custom_invalid() {
    let machine = Machine::new(CpuSetup::custom(1, 0, 2), MemorySetup::new(512), false);
    machine.start().unwrap();
}

#[test]
fn start_one_interface_basic_valid() {
    let mut machine = Machine::new(CpuSetup::new(1), MemorySetup::new(512), false);

    machine.add_interface(TapInterface::new("tamer0"));
    machine.start().unwrap();
}

#[test]
fn start_one_interface_custom_mac_valid() {
    let mut machine = Machine::new(CpuSetup::new(1), MemorySetup::new(512), false);

    machine.add_interface(TapInterface::with_mac_addr("tamer1", "52:54:01:02:03:04"));
    machine.start().unwrap();
}

#[test]
fn start_multiple_interfaces() {
    let mut machine = Machine::new(CpuSetup::new(1), MemorySetup::new(512), false);

    machine.add_interface(TapInterface::new("tamer2"));
    machine.add_interface(TapInterface::new("tamer3"));
    machine.add_interface(TapInterface::new("tamer4"));

    machine.start().unwrap();
}
