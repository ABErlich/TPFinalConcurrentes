pub struct Configuration {
    pub player_count: i8
}


pub fn parse_parameters(params : Vec<String>) -> Configuration {

    if params.len() <= 1 {
        panic!("error: Cantidad insuficiente de parametros");
    }
    
    let player_count: i8 = match params[1].parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            panic!("error: El numero de jugadores invalido");
        },
    };

    // Valido que los jugadores sean pares y >= 4
    if player_count < 4 || player_count % 2 == 1 || player_count > 52{
        panic!("error: El numero de jugadores invalido");
    }

    Configuration {
        player_count: player_count
    }
}




#[test]
#[should_panic]
fn incorrect_parameters_panics_1() {
    let params = Vec::new();
    
    parse_parameters(params);
}


#[test]
#[should_panic]
fn incorrect_parameters_panics_2() {
    let mut params = Vec::new();
    params.push("progName".to_string());
    params.push("3".to_string());
    
    parse_parameters(params);
}

#[test]
fn correct_parameter(){
    let mut params = Vec::new();
    params.push("progName".to_string());
    params.push("4".to_string());

    assert_eq!(parse_parameters(params).player_count, 4);

}