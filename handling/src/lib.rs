// use std::{io::{Write}, path::Path};
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    let mut last_content = String::new();
    let _ = file.read_to_string(&mut last_content);
    println!("{} , {}", last_content, content);
    let _ = file.write_all((content).as_bytes());
}
