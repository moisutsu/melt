use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ext {
    TarGz,
    TarBz2,
    TarXz,
    Tar,
    Zip,
    Other(String),
}

static EXTENTION_MAP: Lazy<HashMap<String, Ext>> = Lazy::new(|| {
    maplit::hashmap! {
        "tar.gz".to_string() => Ext::TarGz,
        "tar.bz2".to_string() => Ext::TarBz2,
        "tar.xz".to_string() => Ext::TarXz,
        "tar".to_string() => Ext::Tar,
        "zip".to_string() => Ext::Zip,
    }
});

pub fn get_extention(file_name: &str) -> Option<Ext> {
    let extention = get_extention_as_string(file_name);

    if extention == "".to_string() {
        None
    } else if EXTENTION_MAP.contains_key(&extention) {
        Some(EXTENTION_MAP[&extention].clone())
    } else {
        Some(Ext::Other(extention))
    }
}

fn get_extention_as_string(file_name: &str) -> String {
    let mut return_extention = "".to_string();
    let mut remain_file_name = file_name.to_string();

    'outer: loop {
        // Update `return_extention` and `remain_file_name`
        // If `remain_file_name` has no extention, return `return_extention`
        let remain_file_path = Path::new(&remain_file_name);
        if let Some(extention) = remain_file_path.extension() {
            return_extention = if return_extention == "".to_string() {
                extention.to_str().unwrap().to_string()
            } else {
                format!("{}.{}", extention.to_str().unwrap(), return_extention)
            };
            remain_file_name = remain_file_path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
        } else {
            return return_extention;
        }

        // If a key in `EXTENTION_MAP` is equal to `return_extention`, return `return_extention`
        for key in EXTENTION_MAP.keys() {
            if key == &return_extention {
                return return_extention;
            }
        }

        // If a key in `EXTENTION_MAP` contains `return_extention`, continue this loop, else return `return_extention`
        for key in EXTENTION_MAP.keys() {
            if key.contains(&return_extention) {
                continue 'outer;
            }
        }
        return return_extention;
    }
}
