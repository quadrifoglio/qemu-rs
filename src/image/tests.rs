use super::Format;

#[test]
fn string_representations() {
    assert_eq!(Format::Raw.to_string().as_str(), "raw");
    assert_eq!(Format::Cloop.to_string().as_str(), "cloop");
    assert_eq!(Format::Cow.to_string().as_str(), "cow");
    assert_eq!(Format::QCow.to_string().as_str(), "qcow");
    assert_eq!(Format::QCow2.to_string().as_str(), "qcow2");
    assert_eq!(Format::Vmdk.to_string().as_str(), "vmdk");
    assert_eq!(Format::Vdi.to_string().as_str(), "vdi");
    assert_eq!(Format::Vhdx.to_string().as_str(), "vhdx");
    assert_eq!(Format::Vpc.to_string().as_str(), "vpc");
}
