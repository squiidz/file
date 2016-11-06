#![feature(proc_macro)]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate chrono;

use std::env;
mod files;

fn main() {
    let pwd = match env::current_dir() {
        Ok(pb) => pb,
        Err(_) => return,
    };

    let file_names = match files::list_files(pwd.as_path()) {
            Some(v) => v,
            None => return,
    };

    println!("{}", serde_json::to_string_pretty(&file_names).unwrap());
}
