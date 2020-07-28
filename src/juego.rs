use super::*;

#[derive(PartialEq)]
pub enum Mensaje {
    JugarNormal,
    JugarRustica,
    FinDelJuego,
    SuspendidoEnRustica
}

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
        let (sender_ronda, receiver_ronda) = channel::<Mensaje>();
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
    let mut cartas = mazo.cartas;
    cartas.shuffle(&mut rng); // Mezclo las cartas

    for i in 0..(cartas_por_jugador * n_jugadores) {
        let carta = cartas[i].clone();

        jugadores_channels_sender[i % n_jugadores].send(carta).unwrap();
        
    }

    barrier.wait();

    sinc::SincronizadorCoordinador {
        jugadores_handler: jugadores, 
        pilon_central_cartas: pilon_central_receiver,
        jugadores_channels: jugadores_channels_sender,
        barrier,
        jugadores_ronda: jugadores_channels_ronda
    }
}


pub fn iniciar_ronda(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, sinc: &sinc::SincronizadorCoordinador, jugador_suspendido: usize) -> ResumenRonda {
    
    let jugadas;
    let puntos;
    let mut jugador_a_suspender: usize = 0;

    if sortear_ronda() > 0.5 {
        logger::log(&log, " -- Iniciando ronda normal --\n".to_string());
        jugadas = ronda_normal(&log, &sinc, jugador_suspendido);
        puntos = contabilizar_puntos(&jugadas);
    } else {
        logger::log(&log, "-- Iniciando ronda rustica --\n".to_string());
        jugadas = ronda_rustica(&log, &sinc, jugador_suspendido);
        let result = contabilizar_puntos_ronda_rustica(&jugadas);
        puntos = result.0;
        jugador_a_suspender = result.1;
        logger::log(&log, format!("Jugador {} suspendido\n", jugador_a_suspender));
    }

    ResumenRonda {
        jugadores_puntos: puntos,
        jugador_suspendido: jugador_a_suspender,
        ultima_ronda: ultima_ronda(&jugadas)
    }


}


fn ronda_normal(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, sinc: &sinc::SincronizadorCoordinador, jugador_suspendido: usize) -> Vec<Jugada> {

    let mut cartas_jugadores: Vec<Jugada> = vec![];

    for i in 0..sinc.jugadores_channels.len() {
        // Le doy el permiso para jugar
        // logger::log(&log, format!("Dandole permiso a {}\n", i + 1));
        if i+1 != jugador_suspendido {
            sinc.jugadores_ronda[i].send(Mensaje::JugarNormal).unwrap();

            // recibo la carta que jugo
            let jugada = sinc.pilon_central_cartas.recv().unwrap();
            logger::log(&log, format!("Coordinador recibi: {} del jugador {}\n", jugada.carta.card_to_string(), jugada.numero_jugador));
            cartas_jugadores.push(jugada);
        }
    }

    cartas_jugadores
}

fn ronda_rustica(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, sinc: &sinc::SincronizadorCoordinador, jugador_suspendido: usize) -> Vec<Jugada>{

    let mut cartas_jugadores: Vec<Jugada> = vec![];

    for i in 0..sinc.jugadores_channels.len() {
        // Le doy el permiso para jugar
        if i+1 != jugador_suspendido {
            sinc.jugadores_ronda[i].send(Mensaje::JugarRustica).unwrap();
        } else {
            sinc.jugadores_ronda[i].send(Mensaje::SuspendidoEnRustica).unwrap()
        }
    }

    sinc.barrier.wait();

    for i in 0..sinc.jugadores_channels.len() {
        // recibo la carta que jugo
        if i+1 != jugador_suspendido {
            let jugada = sinc.pilon_central_cartas.recv().unwrap();
            logger::log(&log, format!("Coordinador recibi: {} del jugador {}\n", jugada.carta.card_to_string(), jugada.numero_jugador));
            cartas_jugadores.push(jugada);
        }
    }

    cartas_jugadores
}


pub fn terminar_juego(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, sinc: &sinc::SincronizadorCoordinador) {

    for i in 0..sinc.jugadores_channels.len() {
        // Le doy el permiso para jugar
        logger::log(&log, format!("Avisandole a {} que se termino el juego\n", i + 1));
        sinc.jugadores_ronda[i].send(Mensaje::FinDelJuego).unwrap();

    }
}


fn sortear_ronda() -> f64 {

    let mut rng = thread_rng();
    rng.gen_range(0., 1.0)
    
}


// Devuelve un vector de tuplas de la forma (numero_jugador, puntos_ganados)
fn contabilizar_puntos(jugadas: &[Jugada]) -> Vec<(usize, f64)> {

    let puntos_a_repartir = 10.;
    let mut cantidad_ganadores = 0.;
    let mut ganadores = Vec::new();
    let mut carta_maxima = &jugadas.first().unwrap().carta;

    // veo cual es la carta maximas
    for jugada in jugadas.iter() {
        if jugada.carta.valor_carta() > carta_maxima.valor_carta() {
            carta_maxima = &jugada.carta;
        }
    }

    // cuantos ganadores tengo
    for jugada in jugadas.iter() {        
        if  jugada.carta.valor_carta() == carta_maxima.valor_carta()  {
            cantidad_ganadores +=  1.;
        }
    }

    // armo el resultado
    for jugada in jugadas.iter() {        
        if  jugada.carta.valor_carta() == carta_maxima.valor_carta() {
            ganadores.push((jugada.numero_jugador, puntos_a_repartir / cantidad_ganadores))
        }
    }
    
    ganadores
}

fn contabilizar_puntos_ronda_rustica(jugadas: &[Jugada]) -> (Vec<(usize, f64)>, usize) {
    const PUNTOS_POR_SALIR_PRIMERO: f64 = 1.0;
    const PUNTOS_POR_SALIR_ULTIMO: f64 = -5.0;

    let mut ganadores = contabilizar_puntos(&jugadas);
    let primer_jugador = jugadas.first().unwrap();
    let ultimo_jugador = jugadas.last().unwrap();


    let idx_primero = ganadores.iter().position(|j| j.0 == primer_jugador.numero_jugador );
    match idx_primero {
        Some(idx_primero) => ganadores[idx_primero].1 += PUNTOS_POR_SALIR_PRIMERO,
        None => ganadores.push((primer_jugador.numero_jugador, PUNTOS_POR_SALIR_PRIMERO))
    }

    let idx_ultimo = ganadores.iter().position(|j| j.0 == ultimo_jugador.numero_jugador );
    match idx_ultimo {
        Some(idx_ultimo) => ganadores[idx_ultimo].1 += PUNTOS_POR_SALIR_ULTIMO,
        None => ganadores.push((ultimo_jugador.numero_jugador, PUNTOS_POR_SALIR_ULTIMO))
    }

    (ganadores, ultimo_jugador.numero_jugador)
}

fn ultima_ronda(jugadas: &[Jugada]) -> bool {

    for j in jugadas{
        if j.cartas_restantes == 0 {
            return true;
        }
    }

    false

}





#[test]
fn contabilizador_puntos_1() {

    let mut jugadas = Vec::new();
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::As, palo: mazo::Palo::Picas}, numero_jugador: 1, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::Dos, palo: mazo::Palo::Picas}, numero_jugador: 2, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::Tres, palo: mazo::Palo::Picas}, numero_jugador: 3, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::Cuatro, palo: mazo::Palo::Picas}, numero_jugador: 4, cartas_restantes: 0 });

    let resultado = contabilizar_puntos(&jugadas);

    assert!(resultado.contains(&(1, 10.)));
}


#[test]
fn contabilizador_puntos_2() {

    let mut jugadas = Vec::new();
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::As, palo: mazo::Palo::Picas}, numero_jugador: 1, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::As, palo: mazo::Palo::Diamantes}, numero_jugador: 2, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::Tres, palo: mazo::Palo::Picas}, numero_jugador: 3, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::Cuatro, palo: mazo::Palo::Picas}, numero_jugador: 4, cartas_restantes: 0 });

    let resultado = contabilizar_puntos(&jugadas);

    assert!(resultado.contains(&(1, 5.)));
}




#[test]
fn contabilizador_puntos_rustica_1() {

    let mut jugadas = Vec::new();
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::As, palo: mazo::Palo::Picas}, numero_jugador: 1, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::Dos, palo: mazo::Palo::Picas}, numero_jugador: 4, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::Tres, palo: mazo::Palo::Picas}, numero_jugador: 3, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::Cuatro, palo: mazo::Palo::Picas}, numero_jugador: 2, cartas_restantes: 0 });

    let resultado = contabilizar_puntos_ronda_rustica(&jugadas);

    assert!(resultado.1 == 2);
    
}


#[test]
fn contabilizador_puntos_rustica_2() {

    let mut jugadas = Vec::new();
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::As, palo: mazo::Palo::Picas}, numero_jugador: 1, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::Dos, palo: mazo::Palo::Picas}, numero_jugador: 4, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::Tres, palo: mazo::Palo::Picas}, numero_jugador: 3, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::Cuatro, palo: mazo::Palo::Picas}, numero_jugador: 2, cartas_restantes: 0 });

    let resultado = contabilizar_puntos_ronda_rustica(&jugadas);
    assert!(resultado.0.contains(&(1, 11.)));
    
}


#[test]
fn contabilizador_puntos_rustica_3() {

    let mut jugadas = Vec::new();
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::As, palo: mazo::Palo::Picas}, numero_jugador: 1, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::Dos, palo: mazo::Palo::Picas}, numero_jugador: 4, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::Tres, palo: mazo::Palo::Picas}, numero_jugador: 3, cartas_restantes: 0 });
    jugadas.push(Jugada { carta: mazo::Carta { numero: mazo::Numero::Cuatro, palo: mazo::Palo::Picas}, numero_jugador: 2, cartas_restantes: 0 });

    let resultado = contabilizar_puntos_ronda_rustica(&jugadas);

    assert!(resultado.0.contains(&(2, -5.)));
    
}
