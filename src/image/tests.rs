use std;
use super::{Format, Image};

/*
 * Verify that the image format's string representation
 * is correct, just the way QEMU expects
 */
#[test]
fn string_representations() {
    assert_eq!(Format::Raw.to_string().as_str(), "raw");
    assert_eq!(Format::QCow.to_string().as_str(), "qcow");
    assert_eq!(Format::QCow2.to_string().as_str(), "qcow2");
    assert_eq!(Format::Vmdk.to_string().as_str(), "vmdk");
    assert_eq!(Format::Vdi.to_string().as_str(), "vdi");
    assert_eq!(Format::Vhdx.to_string().as_str(), "vhdx");
    assert_eq!(Format::Vpc.to_string().as_str(), "vpc");
}

/*
 * Test image creation
 */
#[test]
#[allow(unused_must_use)]
fn create() {
    let img_raw = Image::new("test.raw", Format::Raw, 536_870_912);
    assert!(img_raw.write().is_ok());

    let img_qcow = Image::new("test.qcow", Format::QCow, 536_870_912);
    assert!(img_qcow.write().is_ok());

    let img_qcow2 = Image::new("test.qcow2", Format::QCow2, 536_870_912);
    assert!(img_qcow2.write().is_ok());

    let img_vmdk = Image::new("test.vmdk", Format::Vmdk, 536_870_912);
    assert!(img_vmdk.write().is_ok());

    let img_vdi = Image::new("test.vdi", Format::Vdi, 536_870_912);
    assert!(img_vdi.write().is_ok());

    let img_vhdx = Image::new("test.vhdx", Format::Vhdx, 536_870_912);
    assert!(img_vhdx.write().is_ok());

    let img_vpc = Image::new("test.vpc", Format::Vpc, 536_870_912);
    assert!(img_vpc.write().is_ok());

    // Remove created files
    std::fs::remove_file("test.raw");
    std::fs::remove_file("test.qcow");
    std::fs::remove_file("test.qcow2");
    std::fs::remove_file("test.vmdk");
    std::fs::remove_file("test.vdi");
    std::fs::remove_file("test.vhdx");
    std::fs::remove_file("test.vpc");
}
