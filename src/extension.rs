use once_cell::sync::Lazy;
use std::{collections::HashMap, path::Path};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ext {
    Zip,
    Gz,
    Z,
    Bz2,
    Tar,
    TarGz,
    TarZ,
    TarBz2,
    TarXz,
    Other(String),
}

static EXTENTION_MAP: Lazy<HashMap<String, Ext>> = Lazy::new(|| {
    vec![
        ("zip".to_string(), Ext::Zip),
        ("gz".to_string(), Ext::Gz),
        ("Z".to_string(), Ext::Z),
        ("bz2".to_string(), Ext::Bz2),
        ("tar".to_string(), Ext::Tar),
        ("tar.gz".to_string(), Ext::TarGz),
        ("tgz".to_string(), Ext::TarGz),
        ("tar.Z".to_string(), Ext::TarZ),
        ("taz".to_string(), Ext::TarZ),
        ("tar.bz2".to_string(), Ext::TarBz2),
        ("tbz2".to_string(), Ext::TarBz2),
        ("tar.xz".to_string(), Ext::TarXz),
    ]
    .into_iter()
    .collect()
});

// This function returns the longest matching extension
pub fn get_extention(file_path: &Path) -> Ext {
    let file_name = file_path.to_str().unwrap_or_default();

    // If the name of the file is 'sample.a.b.c.zip', look for the extension 'a.b.c.zip' -> 'b.c.zip' -> 'c.zip' -> 'zip'
    let splited_file_name = file_name.split('.').collect::<Vec<&str>>();
    let mut current_extention = String::new();
    for i in 1..splited_file_name.len() {
        current_extention = splited_file_name[i..].join(".");
        if let Some(extention) = EXTENTION_MAP.get(&current_extention) {
            return extention.clone();
        }
    }
    Ext::Other(current_extention)
}
