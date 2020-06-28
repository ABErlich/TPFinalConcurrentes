mod utilities;

fn main() {

    let config = utilities::parse_parameters(std::env::args().collect());
    println!("player count: {0}", config.player_count);

}

