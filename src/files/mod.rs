#[allow(unused_imports)]
use std::fs::{Metadata, Permissions, read_dir};
use std::path::{PathBuf, Path};
use std::convert::From;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, Debug)]
enum FileType {
    #[serde(rename = "dir")]
    Dir,
    #[serde(rename = "file")]
    File,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "type")]
    file_type: FileType,
    #[serde(rename = "path")]
    file_path: PathBuf,
    #[serde(rename = "name")]
    file_name: String,
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

#[derive(Serialize, Deserialize)]
struct Meta {
    created: NaiveDateTime,
    accessed: NaiveDateTime,
    modified: NaiveDateTime,
    size: u64,
    // permission: Permissions,
}

impl From<Metadata> for Meta {
    fn from(meta_data: Metadata) -> Meta {
        Meta {
            created: convert_system_time(meta_data.created().unwrap()),
            accessed: convert_system_time(meta_data.accessed().unwrap()),
            modified: convert_system_time(meta_data.modified().unwrap()),
            size: meta_data.len(),
            // permission: meta_data.permissions(),
        }
    }
}

fn convert_system_time(sys_time: SystemTime) -> NaiveDateTime {
    let duration = match sys_time.duration_since(UNIX_EPOCH) {
        Ok(d) => d,
        Err(_) => panic!(),
    };
    NaiveDateTime::from_timestamp(duration.as_secs() as i64, 0)
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

pub fn list_files(path: &Path) -> Option<Vec<File>> {
    let files = match read_dir(path) {
        Ok(files) => files,
        Err(_) => return None,
    };
    let mut f_name = Vec::<File>::new();
    for file in files {
        match file {
            Ok(f) => {
                f_name.push(File::from(f.path()));
            },
            Err(_) => continue,
        }
    }
    Some(f_name)
}
