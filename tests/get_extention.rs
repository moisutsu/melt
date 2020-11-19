use melt::{get_extention, Ext};

#[test]
fn test_get_extention() {
    assert_eq!(get_extention("sample.zip"), Some(Ext::Zip));
    assert_eq!(get_extention("sample.tar"), Some(Ext::Tar));
    assert_eq!(get_extention("sample.tar.gz"), Some(Ext::TarGz));
    assert_eq!(get_extention("sample.tar.bz2"), Some(Ext::TarBz2));
    assert_eq!(get_extention("sample.tar.xz"), Some(Ext::TarXz));
    assert_eq!(
        get_extention("sample.rs"),
        Some(Ext::Other("rs".to_string()))
    );
    assert_eq!(
        get_extention("sample.rs.gz"),
        Some(Ext::Other("rs.gz".to_string()))
    );
    assert_eq!(get_extention("sample"), None);
    assert_eq!(get_extention(".sample"), None);
    assert_eq!(
        get_extention(".sample.gz"),
        Some(Ext::Other("gz".to_string()))
    );
}
