use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
pub enum Ext {
    TarGz,
    TarBz2,
    TarXz,
    Tar,
    Zip,
    Other(String),
}

trait InnerExtension {
    fn inner_extention(&self) -> Option<String>;
}

impl InnerExtension for Path {
    fn inner_extention(&self) -> Option<String> {
        let inner_file_name = self.file_stem()?.to_str()?;
        let inner_file_path = Path::new(inner_file_name);
        let inner_extention = inner_file_path.extension()?.to_str()?;
        Some(inner_extention.to_string())
    }
}

pub fn get_extention(file_name: &String) -> Ext {
    let file_path = Path::new(&file_name);

    match file_path.extension() {
        Some(outer_extention) => match outer_extention.to_str().unwrap() {
            "tar" => Ext::Tar,
            "zip" => Ext::Zip,
            outer_ext if ["gz", "bz2", "xz"].contains(&outer_ext) => {
                match &file_path.inner_extention().unwrap()[..] {
                    "tar" if outer_ext == "gz" => Ext::TarGz,
                    "tar" if outer_ext == "bz2" => Ext::TarBz2,
                    "tar" if outer_ext == "xz" => Ext::TarXz,
                    inner_ext => Ext::Other(format!("{}.{}", inner_ext, outer_ext)),
                }
            }
            ext => Ext::Other(ext.to_string()),
        },
        None => Ext::Other("".to_string()),
    }
}
