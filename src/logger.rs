use std::io::prelude::*;
use std::fs::OpenOptions;

pub fn log(message : String) {
    let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open("log.txt").unwrap();


    file.write(message.as_bytes()).unwrap();
    
}