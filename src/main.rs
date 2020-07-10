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
    let n_jugadores = config.player_count as usize;
    let log = logger::crear_log();

    // let mesa = Arc::new(Mutex::new(channel::<mazo::Carta>()));

    logger::log(&log, format!("Cantidad de jugadores: {0}\n", n_jugadores));

    let jugadores = iniciar_juego(&log, n_jugadores);



    



    for jugador in jugadores {
        // Esperar que terminen los threads
        let _ = jugador.join();
    }

}


// Estado inicial, se crean los jugadores y se reparten las cartas
fn iniciar_juego(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, n_jugadores: usize) -> std::vec::Vec<std::thread::JoinHandle<()>> {

    let mut jugadores = vec![];
    let mut jugadores_channels = vec![];
    let mazo = mazo::nuevo();

    let cartas_por_jugador = mazo.cartas.len() / n_jugadores;

    // Lanzo los jugadores
    for i in 1..n_jugadores + 1 {
        let (sender_jugador, receiver_jugador) = channel::<mazo::Carta>();
        jugadores_channels.push(sender_jugador);

        let log = Arc::clone(&log);
        jugadores.push( thread::spawn(move || 
            { 
                jugador::jugador(&log, i, receiver_jugador, cartas_por_jugador);
            }
        ));
    }

    
    // reparto las cartas, TODO: Hacer que se haga un shuffle de cartas
    for i in 0..(cartas_por_jugador * n_jugadores) {
        let carta = mazo.cartas[i].clone();

        jugadores_channels[i % n_jugadores as usize].send(carta).unwrap();
        
    }

    return jugadores;
}