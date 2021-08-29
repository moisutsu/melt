use std::path::PathBuf;

use melt::{get_extention, Ext};

#[test]
fn test_get_extention() {
    assert!(eq_get_extention("sample.zip", Ext::Zip));
    assert!(eq_get_extention("sample.tar", Ext::Tar));
    assert!(eq_get_extention("sample.rs.gz", Ext::Gz));
    assert!(eq_get_extention(".sample.gz", Ext::Gz));
    assert!(eq_get_extention("sample.Z", Ext::Z));
    assert!(eq_get_extention("sample.bz2", Ext::Bz2));
    assert!(eq_get_extention("sample.tar.gz", Ext::TarGz));
    assert!(eq_get_extention("sample.tgz", Ext::TarGz));
    assert!(eq_get_extention("sample.tar.Z", Ext::TarZ));
    assert!(eq_get_extention("sample.taz", Ext::TarZ));
    assert!(eq_get_extention("sample.tar.bz2", Ext::TarBz2));
    assert!(eq_get_extention("sample.tbz2", Ext::TarBz2));
    assert!(eq_get_extention("sample.tar.xz", Ext::TarXz));
    assert!(eq_get_extention("sample.rs", Ext::Other("rs".to_string())));
    assert!(eq_get_extention("sample", Ext::Other("".to_string())));
    assert!(eq_get_extention(
        ".sample",
        Ext::Other("sample".to_string())
    ));
    assert!(eq_get_extention("sample.", Ext::Other("".to_string())));
}

fn eq_get_extention(file_name: &str, extention: Ext) -> bool {
    get_extention(&PathBuf::from(file_name)) == extention
}
