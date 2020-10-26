use melt::{get_extention, Ext};

#[test]
fn test_get_extention() {
    assert_eq!(get_extention(&"sample.zip".to_string()), Some(Ext::Zip));
    assert_eq!(get_extention(&"sample.tar".to_string()), Some(Ext::Tar));
    assert_eq!(
        get_extention(&"sample.tar.gz".to_string()),
        Some(Ext::TarGz)
    );
    assert_eq!(
        get_extention(&"sample.tar.bz2".to_string()),
        Some(Ext::TarBz2)
    );
    assert_eq!(
        get_extention(&"sample.tar.xz".to_string()),
        Some(Ext::TarXz)
    );
    assert_eq!(
        get_extention(&"sample.rs".to_string()),
        Some(Ext::Other("rs".to_string()))
    );
    assert_eq!(
        get_extention(&"sample.rs.gz".to_string()),
        Some(Ext::Other("rs.gz".to_string()))
    );
    assert_eq!(get_extention(&"sample".to_string()), None);
    assert_eq!(get_extention(&".sample".to_string()), None);
    assert_eq!(
        get_extention(&".sample.gz".to_string()),
        Some(Ext::Other("gz".to_string()))
    );
}
