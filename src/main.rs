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
    let mut jugador_suspendido = 0;
    let coordinador_sinc = juego::iniciar_juego(&log, n_jugadores);
    let mut puntajes = Vec::new();

    // Inicializo los puntakes
    for _i in 0..n_jugadores {
        puntajes.push(0.);
    }

    let mut numero_ronda = 1;
    loop {

        logger::log(&log, format!("Iniciando ronda {}.\n", numero_ronda));
        let resumen = juego::iniciar_ronda(&log, &coordinador_sinc, jugador_suspendido);

        for p in resumen.jugadores_puntos {
            puntajes[p.0 - 1] += p.1;
            logger::log(&log, format!("Jugador {}: sacó {:.2} puntos \n", p.0, p.1));
        }

        jugador_suspendido = resumen.jugador_suspendido;
        logger::log(&log, "-- Termino ronda --\n".to_string());

        if resumen.ultima_ronda {
            juego::terminar_juego(&log, &coordinador_sinc);
            break;
        }
        numero_ronda += 1;
    }

    for jugador in coordinador_sinc.jugadores_handler {
        let _ = jugador.join();
    }

    logger::log(&log, "Finalizó el juego, puntajes: \n".to_string());
    for (i, puntaje) in puntajes.iter().enumerate() {
        logger::log(&log, format!("Jugador {} con {:.2} puntos\n", i+1, puntaje))
    }


}













