pub struct Configuration {
    pub player_count: i8
}


pub fn parse_parameters(params : Vec<String>) -> Configuration {

    if params.len() <= 1 {
        eprintln!("error: Cantidad insuficiente de parametros");
        std::process::exit(1);
    }
    
    let player_count: i8 = match params[1].parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("error: El numero de jugadores invalido");
            std::process::exit(1);
        },
    };

    // Valido que los jugadores sean pares y >= 4
    if player_count < 4 || player_count % 2 == 1 {
        eprintln!("error: El numero de jugadores invalido");
        std::process::exit(1);
    }

    Configuration {
        player_count: player_count
    }
}