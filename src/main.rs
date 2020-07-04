mod utilities;
mod logger;
use std::thread;
use std::sync::{Arc};

const CANTIDAD_CARTAS: u8 = 52; // La baraja francesa es un conjunto de naipes o cartas, formado por 52 unidades.

fn main() {
    let config = utilities::parse_parameters(std::env::args().collect());
    let n_jugadores = config.player_count as u8;
    let log = logger::crear_log();
    logger::log(&log, format!("Cantidad de jugadores: {0}\n", n_jugadores));

    let mut children = vec![];

    for i in 1..n_jugadores + 1 {

        let log = Arc::clone(&log);
        children.push( thread::spawn(move || 
            { 
                logger::log(&log, format!("Jugador {}: Tengo {} cartas\n", i, CANTIDAD_CARTAS/n_jugadores)); 
            }
        ));
    }


    for child in children {
        // Esperar que terminen los threads
        let _ = child.join();
    }

}

