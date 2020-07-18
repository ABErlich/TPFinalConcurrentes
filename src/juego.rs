use super::*;


pub struct Jugada {
    pub carta: mazo::Carta,
    pub numero_jugador: usize,
    pub cartas_restantes: usize,
}

pub struct ResumenRonda {
    pub jugadores_puntos: Vec<(usize, f64)>,
    pub jugador_suspendido: usize,
    pub ultima_ronda: bool
}


// Estado inicial, se crean los jugadores y se reparten las cartas
pub fn iniciar_juego(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, n_jugadores: usize) -> sinc::SincronizadorCoordinador {

    let mut jugadores = vec![];
    let mut jugadores_channels_sender = vec![];
    let mut jugadores_channels_ronda = vec![];
    let mazo = mazo::nuevo();
    let barrier = Arc::new(Barrier::new(n_jugadores + 1));
    let (pilon_central_sender, pilon_central_receiver) = channel::<Jugada>();
    let cartas_por_jugador = mazo.cartas.len() / n_jugadores;

    // Lanzo los jugadores
    for i in 1..n_jugadores + 1 {

        let (sender_jugador, receiver_jugador) = channel::<mazo::Carta>();
        let (sender_ronda, receiver_ronda) = channel::<bool>();
        jugadores_channels_sender.push(sender_jugador);
        jugadores_channels_ronda.push(sender_ronda);

        let sinc = sinc::SincronizadorJugador { 
            cartas_receiver: receiver_jugador, 
            pilon_central_cartas: pilon_central_sender.clone(),
            barrier: barrier.clone(),
            ronda_receiver: receiver_ronda
        };

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

    return sinc::SincronizadorCoordinador {
        jugadores_handler: jugadores, 
        pilon_central_cartas: pilon_central_receiver,
        jugadores_channels: jugadores_channels_sender,
        barrier: barrier,
        jugadores_ronda: jugadores_channels_ronda
    };
}


pub fn iniciar_ronda(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, sinc: &sinc::SincronizadorCoordinador) -> ResumenRonda {
    
    let mut jugadas = Vec::new();

    if sortear_ronda() > 0.0 {
        logger::log(&log, "Ronda normal\n".to_string());
        jugadas = ronda_normal(&log, &sinc);
    } else {
        logger::log(&log, "Iniciando ronda rustica\n".to_string());
    }

    let resumen = ResumenRonda {
        jugadores_puntos: contabilizar_puntos(&jugadas),
        jugador_suspendido: 100,
        ultima_ronda: ultima_ronda(&jugadas)
    };


    return resumen;

}


fn ronda_normal(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, sinc: &sinc::SincronizadorCoordinador) -> Vec<Jugada> {

    let mut cartas_jugadores: Vec<Jugada> = vec![];

    for i in 0..sinc.jugadores_channels.len() {
        // Le doy el permiso para jugar
        // logger::log(&log, format!("Dandole permiso a {}\n", i + 1));
        sinc.jugadores_ronda[i].send(true).unwrap();

        // recibo la carta que jugo
        let jugada = sinc.pilon_central_cartas.recv().unwrap();
        logger::log(&log, format!("Coordinador recibi: {} de {} del jugador {}\n", jugada.carta.numero, jugada.carta.palo, jugada.numero_jugador));
        cartas_jugadores.push(jugada);

    }

    return cartas_jugadores;
}

fn _ronda_rustica(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, _cartas: Vec<(mazo::Carta, usize)>) {
    //TODO: Funcionalidad ronda rustica
    logger::log(&log, "Termino ronda rustica\n".to_string());
}


pub fn terminar_juego(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, sinc: &sinc::SincronizadorCoordinador) {

    for i in 0..sinc.jugadores_channels.len() {
        // Le doy el permiso para jugar
        logger::log(&log, format!("Avisandole a {} que se termino el juego\n", i + 1));
        sinc.jugadores_ronda[i].send(false).unwrap();

    }
}


fn sortear_ronda() -> f64 {

    let mut rng = thread_rng();
    let random = rng.gen_range(0., 1.0);
    return random;
    
}


// Devuelve un vector de tuplas de la forma (numero_jugador, puntos_ganados)
fn contabilizar_puntos(jugadas: &Vec<Jugada>) -> Vec<(usize, f64)> {

    let puntos_a_repartir = 10.;
    let mut cantidad_ganadores = 0.;
    let mut ganadores = Vec::new();
    let mut carta_maxima = &jugadas.first().unwrap().carta;

    // veo cual es la carta maximas
    for jugada in jugadas.iter() {
        if jugada.carta.valor > carta_maxima.valor {
            carta_maxima = &jugada.carta;
        }
    }

    // cuantos ganadores tengo
    for jugada in jugadas.iter() {        
        if  jugada.carta.numero == carta_maxima.numero  {
            cantidad_ganadores +=  1.;
        }
    }

    // armo el resultado
    for jugada in jugadas.iter() {        
        if  jugada.carta.numero == carta_maxima.numero {
            ganadores.push((jugada.numero_jugador, puntos_a_repartir / cantidad_ganadores))
        }
    }
    
    return ganadores;
}

fn ultima_ronda(jugadas: &Vec<Jugada>) -> bool {

    for j in jugadas{
        if j.cartas_restantes == 0 {
            return true;
        }
    }

    return false;
}