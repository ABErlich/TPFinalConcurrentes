use std::io::prelude::*;
use std::fs::OpenOptions;
use chrono::prelude::*;
use std::sync::{Mutex, Arc};


pub fn crear_log() -> std::sync::Arc<std::sync::Mutex<std::fs::File>> {
    
    Arc::new(Mutex::new(
        match OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open("log.txt") {
                Ok(f)  => f,
                Err(err) => { panic!("No se pude crear log: {}", err); }
            }
    ))
}

pub fn log(file : &std::sync::Arc<std::sync::Mutex<std::fs::File>> , message : String) {

    let local = Local::now().format("%d/%m/%Y %H:%M:%S");

    let output = format!("{} - {}", local, message);

    // Bloque protegido
    {
        let mut f = file.lock().unwrap();
        match f.write(output.as_bytes()){
            Ok(_)  => {},
            Err(err) => println!("Error al loggear: {}", err)
        };
    }
}