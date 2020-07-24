use super::*;

pub fn jugador(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, numero_jugador: usize, organizador: sinc::SincronizadorJugador, cant_cartas: usize) {
    
    let mis_cartas = recibir_cartas(&organizador, cant_cartas);

    logger::log(&log, format!("JUGADOR {} LISTO\n", numero_jugador));
    organizador.barrier.wait(); // aviso que termino de recibir las cartas
    
    let mut next_card = 0;
    loop {
        
        let permiso = esperar_permiso(&organizador);

        // Veo que hago en funcion del mensaje recibido
        match permiso {
            juego::Mensaje::JugarNormal => jugar_carta(&(mis_cartas[next_card]), &organizador, numero_jugador, mis_cartas.len() - next_card - 1),
            juego::Mensaje::JugarRustica => {
                organizador.barrier.wait();
                jugar_carta(&(mis_cartas[next_card]), &organizador, numero_jugador, mis_cartas.len() - next_card - 1);
            },
            juego::Mensaje::SuspendidoEnRustica => {
                organizador.barrier.wait();
            }
            juego::Mensaje::FinDelJuego => break
        };
        
        next_card += 1;
    }
}


fn recibir_cartas(organizador: &sinc::SincronizadorJugador, cant_cartas: usize) -> Vec<mazo::Carta> {

    let mut cartas = Vec::new();

    for _i in 0..cant_cartas {
        let carta = organizador.cartas_receiver.recv().unwrap();
        //logger::log(&log, format!("Jugador {} recibi: {} de {}\n", numero_jugador, carta.numero, carta.palo));
        cartas.push(carta);
    }

    return cartas

}

fn esperar_permiso(organizador: &sinc::SincronizadorJugador) -> juego::Mensaje {
    let permiso = organizador.ronda_receiver.recv().unwrap();

    return permiso;
}

fn jugar_carta(carta: &mazo::Carta, organizador: &sinc::SincronizadorJugador, numero_jugador: usize, cartas_restantes: usize) {
    organizador.pilon_central_cartas.send(
        juego::Jugada { carta: carta.clone(), numero_jugador: numero_jugador, cartas_restantes: cartas_restantes}
    ).unwrap();
}