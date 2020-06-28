
struct Configuration {
    player_count: i32
}

fn main() {

    let config = parse_parameters(std::env::args().collect());
    println!("player count: {0}", config.player_count);

}


fn parse_parameters(params : Vec<String>) -> Configuration {

    let player_count: i32 = match params[1].parse() {
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