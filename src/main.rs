mod utilities;
mod logger;
mod mazo;
mod jugador;
mod juego;
use std::thread;
use std::sync::{Arc, Barrier};
use std::sync::mpsc::channel;
use rand::prelude::*;

fn main() {
    let config = utilities::parse_parameters(std::env::args().collect());
    let n_jugadores = config.player_count as usize;
    let log = logger::crear_log();
    let mut round_number = 1;
    

    // let mesa = Arc::new(Mutex::new(channel::<mazo::Carta>()));

    logger::log(&log, format!("Cantidad de jugadores: {0}\n", n_jugadores));

    let jugadores = juego::iniciar_juego(&log, n_jugadores);

    loop {
        round_number += 1;
        
        juego::iniciar_ronda(&log);


        // TODO: Cambiar la condicion de corte
        if round_number == 5 {
            break;
        }
    }


    for jugador in jugadores {
        // Esperar que terminen los threads
        let _ = jugador.join();
    }

}













