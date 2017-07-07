use image::{Image, Format};
use super::{Drive, DriveMedia, Memory, Machine};

#[test]
fn drive_media_representations() {
    assert_eq!(DriveMedia::CDRom.to_string().as_str(), "cdrom");
    assert_eq!(DriveMedia::Disk.to_string().as_str(), "disk");
}

#[test]
fn startup() {
    let machine = Machine::new(Memory::new(536_870_912), false);
}
