use super::*;

pub fn jugador(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, numero_jugador: usize, organizador: sinc::SincronizadorJugador, cant_cartas: usize) {
    let mut mis_cartas = Vec::new();

    for _i in 0..cant_cartas {
        let carta = organizador.cartas_receiver.recv().unwrap();
        logger::log(&log, format!("Jugador {} recibi: {} de {}\n", numero_jugador, carta.numero, carta.palo));
        mis_cartas.push(carta);
    }

    logger::log(&log, format!("JUGADOR {} LISTO\n", numero_jugador));
    organizador.barrier.wait(); // aviso que termino de recibir las cartas
    
    let mut round_number = 1;
    loop {

        let tipo_de_ronda = organizador.ronda_receiver.recv().unwrap();
        if tipo_de_ronda == false {
            //TODO: sleep para simular tiempo que tarda el jugador en poner la carta.
        }
        organizador.pilon_central_cartas.send((mis_cartas[round_number-1].clone(), numero_jugador)).unwrap();

        if round_number == 4 {
            break;
        }
        round_number += 1;
    }
}