use super::*;


pub struct SincronizadorCoordinador {
    pub jugadores_handler: Vec<thread::JoinHandle<()>>,
    pub jugadores_channels: Vec<Sender<mazo::Carta>>,
    pub jugadores_ronda: Vec<Sender<juego::Mensaje>>,
    pub pilon_central_cartas: Receiver<juego::Jugada>,
    pub barrier: Arc<Barrier>
}

pub struct SincronizadorJugador{
    pub cartas_receiver: Receiver<mazo::Carta>,
    pub ronda_receiver: Receiver<juego::Mensaje>,
    pub pilon_central_cartas: Sender<juego::Jugada>,
    pub barrier: Arc<Barrier>
}