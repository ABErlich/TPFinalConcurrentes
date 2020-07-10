use super::*;

// Estado inicial, se crean los jugadores y se reparten las cartas
pub fn iniciar_juego(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, n_jugadores: usize) -> std::vec::Vec<std::thread::JoinHandle<()>> {

    let mut jugadores = vec![];
    let mut jugadores_channels = vec![];
    let mazo = mazo::nuevo();
    let barrier = Arc::new(Barrier::new(n_jugadores + 1));

    let cartas_por_jugador = mazo.cartas.len() / n_jugadores;

    // Lanzo los jugadores
    for i in 1..n_jugadores + 1 {
        let (sender_jugador, receiver_jugador) = channel::<mazo::Carta>();
        let c = barrier.clone();
        jugadores_channels.push(sender_jugador);

        let log = Arc::clone(&log);
        jugadores.push( thread::spawn(move || 
            { 
                jugador::jugador(&log, i, receiver_jugador, cartas_por_jugador);
                c.wait(); // aviso que termino de recibir las cartas
            }
        ));
    }

    let mut rng = thread_rng();
    let mut cartas = mazo.cartas.clone();
    cartas.shuffle(&mut rng); // Mezclo las cartas

    for i in 0..(cartas_por_jugador * n_jugadores) {
        let carta = cartas[i].clone();

        jugadores_channels[i % n_jugadores].send(carta).unwrap();
        
    }

    barrier.wait();

    return jugadores;
}


pub fn iniciar_ronda(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>) {
    if sortear_ronda() > 0.5 {
        ronda_normal(&log);
    } else {
        ronda_rustica(&log);
    }
}


fn ronda_normal(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>) {
    logger::log(&log, "Iniciando ronda normal\n".to_string());
}

fn ronda_rustica(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>) {
    logger::log(&log, "Iniciando ronda rustica\n".to_string());
}



fn sortear_ronda() -> f64 {

    let mut rng = thread_rng();
    return rng.gen_range(0., 1.0);
    
}