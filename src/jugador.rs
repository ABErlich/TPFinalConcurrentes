use super::mazo;

pub fn jugador(_file : &std::sync::Arc<std::sync::Mutex<std::fs::File>>, _organizador: std::sync::mpsc::Receiver<mazo::Carta>) {
    println!("JUGADOR LISTO");
}