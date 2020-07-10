use super::*;

pub fn jugador(log : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, numero_jugador: usize, organizador: std::sync::mpsc::Receiver<mazo::Carta>, cant_cartas: usize) {
    
    for _i in 0..cant_cartas {
        let carta = organizador.recv().unwrap();
        println!("Jugador {} recibi: {} - {}", numero_jugador, carta.numero, carta.palo);
    }

    println!("JUGADOR {} LISTO", numero_jugador);
}