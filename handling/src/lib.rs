use std::{fs::OpenOptions, io::{Read, Write}, path::Path};

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path).unwrap();
    let mut last_content  = vec![];
    let _ = file.read_to_end(&mut last_content);
    for i in content.as_bytes() {
        last_content.push(*i);

    }
    file.write(&last_content).unwrap();
    // file.write_all(content.as_bytes()).unwrap();
    
}
