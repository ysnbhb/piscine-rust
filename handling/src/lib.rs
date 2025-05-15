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
    file.write((last_content + content).as_bytes()).unwrap();
}
