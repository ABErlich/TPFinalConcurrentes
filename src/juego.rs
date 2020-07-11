use super::*;

pub struct SincronizadorCoordinador {
    pub jugadores_handler: Vec<thread::JoinHandle<()>>,
    pub jugadores_sender: Vec<Receiver<mazo::Carta>>,
}

pub struct SincronizadorJugador{
    pub cartas_receiver: Receiver<mazo::Carta>,
    pub cartas_sender: Sender<mazo::Carta>,
    pub barrier: Arc<Barrier>
}

// Estado inicial, se crean los jugadores y se reparten las cartas
pub fn iniciar_juego(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, n_jugadores: usize) -> SincronizadorCoordinador {

    let mut jugadores = vec![];
    let mut jugadores_channels_sender = vec![];
    let mut jugadores_channels_receiver = vec![];
    let mazo = mazo::nuevo();
    let barrier = Arc::new(Barrier::new(n_jugadores + 1));

    let cartas_por_jugador = mazo.cartas.len() / n_jugadores;

    // Lanzo los jugadores
    for i in 1..n_jugadores + 1 {
        let (sender_jugador, receiver_jugador) = channel::<mazo::Carta>();
        let (sender_coordinador, receiver_coordinador) = channel::<mazo::Carta>();
        jugadores_channels_sender.push(sender_jugador);
        jugadores_channels_receiver.push(receiver_coordinador);
        let sinc = SincronizadorJugador{ cartas_receiver: receiver_jugador, 
            cartas_sender: sender_coordinador,
            barrier: barrier.clone()};

        let log = Arc::clone(&log);
        jugadores.push( thread::spawn(move || 
            { 
                jugador::jugador(&log, i, sinc, cartas_por_jugador);
            }
        ));
    }

    let mut rng = thread_rng();
    let mut cartas = mazo.cartas.clone();
    cartas.shuffle(&mut rng); // Mezclo las cartas

    for i in 0..(cartas_por_jugador * n_jugadores) {
        let carta = cartas[i].clone();

        jugadores_channels_sender[i % n_jugadores].send(carta).unwrap();
        
    }

    barrier.wait();

    return SincronizadorCoordinador{jugadores_handler: jugadores, 
                                jugadores_sender: jugadores_channels_receiver};
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