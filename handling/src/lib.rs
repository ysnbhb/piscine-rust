use std::{fs::File, io::Write, path::Path};

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let file = File::open(path);
    match file {
        Ok(mut f) => {
            f.write(content.as_bytes()).unwrap();
        }
        Err(_) => {
            let f = File::create(path);
            f.unwrap().write(content.as_bytes()).unwrap();
        }
    }
    // todo!()
}
