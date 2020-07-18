mod utilities;
mod logger;
mod mazo;
mod jugador;
mod juego;
mod sinc;
use std::thread;
use std::sync::{Arc, Barrier};
use std::sync::mpsc::{channel, Receiver, Sender};
use rand::prelude::*;

fn main() {
    let config = utilities::parse_parameters(std::env::args().collect());
    let n_jugadores = config.player_count as usize;
    let log = logger::crear_log();
    let mut round_number = 1;
    
    logger::log(&log, format!("Cantidad de jugadores: {0}\n", n_jugadores));

    let (coordinador_sinc, _cant_rondas) = juego::iniciar_juego(&log, n_jugadores);
    //TODO: Devolver un struct con los jugadores y sus canales de comunicacion de cartas.

    loop {
        
        juego::iniciar_ronda(&log, &coordinador_sinc);

        // TODO: Cambiar la condicion de corte
        if round_number == 4 {
            break;
        }
        round_number += 1;
    }


    for jugador in coordinador_sinc.jugadores_handler {
        let _ = jugador.join();
    }

}













