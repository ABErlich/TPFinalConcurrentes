use std::io::prelude::*;
use std::fs::OpenOptions;
use chrono::prelude::*;




pub fn log(message : String) {
    let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open("log.txt").unwrap();

    let local = Local::now().format("%d/%m/%Y %H:%M:%S");

    let output = format!("{} - {}", local, message);
    file.write(output.as_bytes()).unwrap();
    
}