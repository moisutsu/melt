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

pub fn get_extention(file_name: &String) -> Option<Ext> {
    let file_path = Path::new(&file_name);

    let outer_extention = file_path.extension()?.to_str()?;
    let ret_extention = match outer_extention {
        "tar" => Ext::Tar,
        "zip" => Ext::Zip,
        _ if ["gz", "bz2", "xz"].contains(&outer_extention) => {
            if let Some(inner_extention) = file_path.inner_extention() {
                match &inner_extention[..] {
                    "tar" if outer_extention == "gz" => Ext::TarGz,
                    "tar" if outer_extention == "bz2" => Ext::TarBz2,
                    "tar" if outer_extention == "xz" => Ext::TarXz,
                    _ => Ext::Other(format!("{}.{}", inner_extention, outer_extention)),
                }
            } else {
                Ext::Other(outer_extention.to_string())
            }
        }
        _ => Ext::Other(outer_extention.to_string()),
    };
    Some(ret_extention)
}
