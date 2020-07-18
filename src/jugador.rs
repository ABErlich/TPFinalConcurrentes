use super::*;

pub fn jugador(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, numero_jugador: usize, organizador: sinc::SincronizadorJugador, cant_cartas: usize) {
    
    let mis_cartas = recibir_cartas(&organizador, cant_cartas);

    logger::log(&log, format!("JUGADOR {} LISTO\n", numero_jugador));
    organizador.barrier.wait(); // aviso que termino de recibir las cartas
    
    let mut round_number = 1;
    loop {
        
        esperar_permiso(&organizador);

        jugar_carta(&(mis_cartas[round_number-1]), &organizador, numero_jugador);
        // if tipo_de_ronda == false {
        //     //TODO: sleep para simular tiempo que tarda el jugador en poner la carta.
        // }
        
        if round_number == 4 {
            break;
        }
        round_number += 1;
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

fn esperar_permiso(organizador: &sinc::SincronizadorJugador) {
    let _tipo_de_ronda = organizador.ronda_receiver.recv().unwrap();
}

fn jugar_carta(carta: &mazo::Carta, organizador: &sinc::SincronizadorJugador, numero_jugador: usize) {
    organizador.pilon_central_cartas.send((carta.clone(), numero_jugador)).unwrap();
}