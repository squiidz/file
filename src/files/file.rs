use std::fs::{read_dir};
use std::path::{PathBuf, Path};
use std::convert::From;
use std::io::{Error};

use super::meta::{Meta};

#[derive(Serialize, Deserialize, Debug)]
enum FileType {
    #[serde(rename = "dir")]
    Dir,
    #[serde(rename = "file")]
    File,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "name")]
    file_name: String,
    #[serde(rename = "type")]
    file_type: FileType,
    #[serde(rename = "path")]
    file_path: PathBuf,
    #[serde(rename = "meta")]
    file_meta: Meta,
}


impl From<PathBuf> for File {
    fn from(path_buf: PathBuf) -> File {
        File{
            file_type: if path_buf.is_dir() {FileType::Dir} else {FileType::File},
            file_path: path_buf.to_path_buf(),
            file_name: path_buf.file_name()
                .and_then(|x| x.to_str()).unwrap().to_owned(),
            file_meta: Meta::from(path_buf.metadata().unwrap()),
        }
    }
}

#[allow(dead_code)]
fn render_files_list(file_list: Vec<File>) {
    for f in file_list {
        match f.file_type {
            FileType::Dir => print!("<{}>  ", f.file_name),
            _ => print!("{}  ", f.file_name),
        }
    }
}

pub fn list_files(path: &Path) -> Result<Vec<File>, Error> {
    let files = match read_dir(path) {
        Ok(files) => files,
        Err(e) => return Err(e),
    };
    let mut f_name = Vec::<File>::new();
    for file in files {
        match file {
            Ok(f) => {
                f_name.push(File::from(f.path()));
            },
            Err(e) => return Err(e),
        }
    }
    Ok(f_name)
}
