use std::thread;

const CANTIDAD_CARTAS: u8 = 52; //La baraja francesa es un conjunto de naipes o cartas, formado por 52 unidades.
const N_JUGADORES: u8 = 6;

fn main() {
    //let mut players = vec![];
    println!("Cantidad de cartas: {}", CANTIDAD_CARTAS);

    let mut children = vec![];

    for i in 1..N_JUGADORES + 1 {
    	children.push( thread::spawn(move || 
    		{ 
    			println!("Jugador {}: Tengo {} cartas", i, CANTIDAD_CARTAS/N_JUGADORES); 
    		}
    	));
    }


    for child in children {
		// Esperar que terminen los threads
		let _ = child.join();
	}

}
