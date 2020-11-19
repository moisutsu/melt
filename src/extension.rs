use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ext {
    Zip,
    Gz,
    Tar,
    TarGz,
    TarBz2,
    TarXz,
    Other,
}

static EXTENTION_MAP: Lazy<HashMap<String, Ext>> = Lazy::new(|| {
    maplit::hashmap! {
        "zip".to_string() => Ext::Zip,
        "gz".to_string() => Ext::Gz,
        "tar".to_string() => Ext::Tar,
        "tar.gz".to_string() => Ext::TarGz,
        "tar.bz2".to_string() => Ext::TarBz2,
        "tar.xz".to_string() => Ext::TarXz,
    }
});

// This function returns the longest matching extension
pub fn get_extention(file_name: &str) -> Ext {
    let splited_file_name = file_name.split(".").collect::<Vec<&str>>();
    for i in 1..splited_file_name.len() {
        let joined_extention = splited_file_name[i..].join(".");
        if EXTENTION_MAP.keys().any(|key| key == &joined_extention) {
            return EXTENTION_MAP[&joined_extention].clone();
        }
    }
    Ext::Other
}
