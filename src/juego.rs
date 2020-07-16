use super::*;

pub struct SincronizadorCoordinador {
    pub jugadores_handler: Vec<thread::JoinHandle<()>>,
    pub jugadores_channels: Vec<Sender<mazo::Carta>>,
    pub jugadores_ronda: Vec<Sender<bool>>,
    pub pilon_central_cartas: Receiver<(mazo::Carta, usize)>,
    pub barrier: Arc<Barrier>
}

pub struct SincronizadorJugador{
    pub cartas_receiver: Receiver<mazo::Carta>,
    pub ronda_receiver: Receiver<bool>,
    pub pilon_central_cartas: Sender<(mazo::Carta, usize)>,
    pub barrier: Arc<Barrier>
}

// Estado inicial, se crean los jugadores y se reparten las cartas
pub fn iniciar_juego(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, n_jugadores: usize) -> (SincronizadorCoordinador, usize) {

    let mut jugadores = vec![];
    let mut jugadores_channels_sender = vec![];
    let mut jugadores_channels_ronda = vec![];
    let mazo = mazo::nuevo();
    let barrier = Arc::new(Barrier::new(n_jugadores + 1));
    let (pilon_central_sender, pilon_central_receiver) = channel::<(mazo::Carta, usize)>();
    let cartas_por_jugador = mazo.cartas.len() / n_jugadores;

    // Lanzo los jugadores
    for i in 1..n_jugadores + 1 {
        let (sender_jugador, receiver_jugador) = channel::<mazo::Carta>();
        let (sender_ronda, receiver_ronda) = channel::<bool>();
        jugadores_channels_sender.push(sender_jugador);
        jugadores_channels_ronda.push(sender_ronda);
        let sinc = SincronizadorJugador{ cartas_receiver: receiver_jugador, 
            pilon_central_cartas: pilon_central_sender.clone(),
            barrier: barrier.clone(),
            ronda_receiver: receiver_ronda};

        let log = Arc::clone(&log);
        jugadores.push( thread::spawn(move || 
            { 
                jugador::jugador(&log, i, sinc, cartas_por_jugador, cartas_por_jugador);
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

    return (SincronizadorCoordinador{jugadores_handler: jugadores, 
                                pilon_central_cartas: pilon_central_receiver,
                                jugadores_channels: jugadores_channels_sender,
                                barrier: barrier,
                                jugadores_ronda: jugadores_channels_ronda}, cartas_por_jugador);
}


pub fn iniciar_ronda(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, sinc: &SincronizadorCoordinador) {
    let tipo_de_ronda : bool = sortear_ronda(&log);

    for send_ronda_player in sinc.jugadores_ronda.iter(){
        send_ronda_player.send(tipo_de_ronda).unwrap();
    }

    let mut cartas_jugadores: Vec<(mazo::Carta, usize)> = vec![];

    for _i in 0..sinc.jugadores_channels.len() {
        let carta = sinc.pilon_central_cartas.recv().unwrap();
        logger::log(&log, format!("Coordinador recibi: {} de {} del jugador {}\n", carta.0.numero, carta.0.palo, carta.1));
        cartas_jugadores.push(carta);
    }

    if tipo_de_ronda {
        ronda_normal(&log, cartas_jugadores);
    }else{
        ronda_rustica(&log, cartas_jugadores);
    }

}


fn ronda_normal(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, _cartas: Vec<(mazo::Carta, usize)>) {
    //TODO: Funcionalidad ronda normal
    logger::log(&log, "Termino ronda normal\n".to_string());
}

fn ronda_rustica(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, _cartas: Vec<(mazo::Carta, usize)>) {
    //TODO: Funcionalidad ronda rustica
    logger::log(&log, "Termino ronda rustica\n".to_string());
}



fn sortear_ronda(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>) -> bool {

    let mut rng = thread_rng();
    let random = rng.gen_range(0., 1.0);

    if random > 0.5 {
        logger::log(&log, "Iniciando ronda normal\n".to_string());
        return true;
    } else {
        logger::log(&log, "Iniciando ronda rustica\n".to_string());
        return false;
    }
    
}