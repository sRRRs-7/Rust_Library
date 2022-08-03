
use std::{fs::File, io::{ErrorKind, Write}};

pub fn file_process() {
    let path = "text.txt";
    // let f = File::open(path);
    let f = File::create(path);
    let mut f = match f {
        Ok(f) => f,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            match File::create(path) {
                Ok(nf) => nf,
                Err(e) => panic!("Error creating file: {}", e),
            }
        }
        Err(e) => panic!("Error reading file: {}", e),
    };

    let data = b"some bytes";
    let mut pos = 0;

    while pos < data.len() {
        let bytes_written = f.write(&data[pos..]).unwrap();
        pos += bytes_written;
    }
}