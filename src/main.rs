mod utilities;
mod logger;
mod mazo;
mod jugador;
use std::thread;
use std::sync::{Arc};
use std::sync::mpsc::channel;

//const CANTIDAD_CARTAS: u8 = 52; // La baraja francesa es un conjunto de naipes o cartas, formado por 52 unidades.

fn main() {
    let config = utilities::parse_parameters(std::env::args().collect());
    let n_jugadores = config.player_count as u8;
    let log = logger::crear_log();

    // let mesa = Arc::new(Mutex::new(channel::<mazo::Carta>()));

    logger::log(&log, format!("Cantidad de jugadores: {0}\n", n_jugadores));

    let mazo = mazo::nuevo();

    let jugadores = iniciar_juego(&log, n_jugadores);



    // reparto las cartas, TODO: Hacer que se haga un shuffle de cartas
    for carta in mazo.cartas.iter() {
        println!("Numero: {}, palo: {}", carta.numero, carta.palo);
    }



    for jugador in jugadores {
        // Esperar que terminen los threads
        let _ = jugador.join();
    }

}


// Estado inicial, se crean los jugadores y se reparten las cartas
fn iniciar_juego(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, n_jugadores: u8) -> std::vec::Vec<std::thread::JoinHandle<()>> {

    let mut jugadores = vec![];
    // Lanzo los jugadores
    for _i in 1..n_jugadores + 1 {
        let (_sender_jugador, receiver_jugador) = channel::<mazo::Carta>();

        let log = Arc::clone(&log);
        jugadores.push( thread::spawn(move || 
            { 
                jugador::jugador(&log, receiver_jugador);
            }
        ));
    }

    return jugadores;
}