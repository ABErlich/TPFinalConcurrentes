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

    let coordinador_sinc = juego::iniciar_juego(&log, n_jugadores);

    loop {
        
        let puntos = juego::iniciar_ronda(&log, &coordinador_sinc);

        for p in puntos {
            logger::log(&log, format!("Jugador: {} puntos {}\n", p.0, p.1));
        }

        // TODO: Cambiar la condicion de corte
        if round_number == 4 {
            juego::terminar_juego(&log, &coordinador_sinc);
            break;
        }
        round_number += 1;
    }


    for jugador in coordinador_sinc.jugadores_handler {
        let _ = jugador.join();
    }

}













