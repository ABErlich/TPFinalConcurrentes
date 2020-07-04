mod utilities;
mod logger;
use std::thread;

const CANTIDAD_CARTAS: u8 = 52; //La baraja francesa es un conjunto de naipes o cartas, formado por 52 unidades.

fn main() {
    let config = utilities::parse_parameters(std::env::args().collect());
    let n_jugadores = config.player_count as u8;
    logger::log(format!("player count: {0}\n", n_jugadores));

    let mut children = vec![];

    for i in 1..n_jugadores + 1 {
        children.push( thread::spawn(move || 
            { 
                logger::log(format!("Jugador {}: Tengo {} cartas\n", i, CANTIDAD_CARTAS/n_jugadores)); 
            }
        ));
    }


    for child in children {
        // Esperar que terminen los threads
        let _ = child.join();
    }

}

