use super::*;

pub fn jugador(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, numero_jugador: usize, organizador: juego::SincronizadorJugador, cant_cartas: usize) {
    let mut mis_cartas = Vec::new();

    for _i in 0..cant_cartas {
        let carta = organizador.cartas_receiver.recv().unwrap();
        logger::log(&log, format!("Jugador {} recibi: {} de {}\n", numero_jugador, carta.numero, carta.palo));
        mis_cartas.push(carta);
    }

    logger::log(&log, format!("JUGADOR {} LISTO\n", numero_jugador));
    organizador.barrier.wait(); // aviso que termino de recibir las cartas

   
    organizador.cartas_receiver.recv().unwrap();
    organizador.pilon_central_cartas.send(mis_cartas[0].clone()).unwrap();
    
}