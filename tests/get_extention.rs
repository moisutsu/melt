use melt::{get_extention, Ext};

#[test]
fn test_get_extention() {
    assert_eq!(get_extention("sample.zip"), Ext::Zip);
    assert_eq!(get_extention("sample.tar"), Ext::Tar);
    assert_eq!(get_extention("sample.rs.gz"), Ext::Gz);
    assert_eq!(get_extention(".sample.gz"), Ext::Gz);
    assert_eq!(get_extention("sample.Z"), Ext::Z);
    assert_eq!(get_extention("sample.bz2"), Ext::Bz2);
    assert_eq!(get_extention("sample.tar.gz"), Ext::TarGz);
    assert_eq!(get_extention("sample.tar.Z"), Ext::TarZ);
    assert_eq!(get_extention("sample.tar.bz2"), Ext::TarBz2);
    assert_eq!(get_extention("sample.tar.xz"), Ext::TarXz);
    assert_eq!(get_extention("sample.rs"), Ext::Other);
    assert_eq!(get_extention("sample"), Ext::Other);
    assert_eq!(get_extention(".sample"), Ext::Other);
}
