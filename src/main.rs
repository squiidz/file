#![feature(proc_macro)]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


use std::fs;
use std::path::{PathBuf, Path};
use std::env;
use std::convert::From;

#[derive(Serialize, Deserialize, Debug)]
enum FileType {
    Dir,
    File,
}

#[derive(Serialize, Deserialize, Debug)]
struct File {
    file_type: FileType,
    file_path: PathBuf,
    file_name: String,
}

impl From<PathBuf> for File {
    fn from(path_buf: PathBuf) -> File {
        File{
            file_type: if path_buf.is_dir() {FileType::Dir} else {FileType::File},
            file_path: path_buf.to_path_buf(),
            file_name: path_buf.file_name()
                .and_then(|x| x.to_str()).unwrap().to_owned(),
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

fn list_files(path: &Path) -> Option<Vec<File>> {
    let files = match fs::read_dir(path) {
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

fn main() {
    let pwd = match env::current_dir() {
        Ok(pb) => pb,
        Err(_) => return,
    };

    let file_names = match list_files(pwd.as_path()) {
            Some(v) => v,
            None => return,
    };

    println!("{}", serde_json::to_string_pretty(&file_names).unwrap());
}
