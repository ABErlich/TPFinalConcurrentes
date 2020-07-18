#[derive(Clone)]
pub struct Carta {
    pub numero: String,
    pub palo: String,
    pub valor: u8
}


pub struct Mazo {
    pub cartas: std::vec::Vec::<Carta>,
}


pub fn nuevo() -> Mazo {

    let cartas = vec![
        Carta { numero: "As".to_string(), palo: "diamantes".to_string(), valor: 100},
        Carta { numero: "2".to_string(), palo: "diamantes".to_string(), valor: 2},
        Carta { numero: "3".to_string(), palo: "diamantes".to_string(), valor: 3},
        Carta { numero: "4".to_string(), palo: "diamantes".to_string(), valor: 4},
        Carta { numero: "5".to_string(), palo: "diamantes".to_string(), valor: 5},
        Carta { numero: "6".to_string(), palo: "diamantes".to_string(), valor: 6},
        Carta { numero: "7".to_string(), palo: "diamantes".to_string(), valor: 7},
        Carta { numero: "8".to_string(), palo: "diamantes".to_string(), valor: 8},
        Carta { numero: "9".to_string(), palo: "diamantes".to_string(), valor: 9},
        Carta { numero: "10".to_string(), palo: "diamantes".to_string(), valor: 10},
        Carta { numero: "J".to_string(), palo: "diamantes".to_string(), valor: 11},
        Carta { numero: "Q".to_string(), palo: "diamantes".to_string(), valor: 12},
        Carta { numero: "K".to_string(), palo: "diamantes".to_string(), valor: 13},
        
        Carta { numero: "As".to_string(), palo: "picas".to_string(), valor: 100},
        Carta { numero: "2".to_string(), palo: "picas".to_string(), valor: 2},
        Carta { numero: "3".to_string(), palo: "picas".to_string(), valor: 3},
        Carta { numero: "4".to_string(), palo: "picas".to_string(), valor: 4},
        Carta { numero: "5".to_string(), palo: "picas".to_string(), valor: 5},
        Carta { numero: "6".to_string(), palo: "picas".to_string(), valor: 6},
        Carta { numero: "7".to_string(), palo: "picas".to_string(), valor: 7},
        Carta { numero: "8".to_string(), palo: "picas".to_string(), valor: 8},
        Carta { numero: "9".to_string(), palo: "picas".to_string(), valor: 9},
        Carta { numero: "10".to_string(), palo: "picas".to_string(), valor: 10},
        Carta { numero: "J".to_string(), palo: "picas".to_string(), valor: 11},
        Carta { numero: "Q".to_string(), palo: "picas".to_string(), valor: 12},
        Carta { numero: "K".to_string(), palo: "picas".to_string(), valor: 13},

        Carta { numero: "As".to_string(), palo: "corazones".to_string(), valor: 100},
        Carta { numero: "2".to_string(), palo: "corazones".to_string(), valor: 2},
        Carta { numero: "3".to_string(), palo: "corazones".to_string(), valor: 3},
        Carta { numero: "4".to_string(), palo: "corazones".to_string(), valor: 4},
        Carta { numero: "5".to_string(), palo: "corazones".to_string(), valor: 5},
        Carta { numero: "6".to_string(), palo: "corazones".to_string(), valor: 6},
        Carta { numero: "7".to_string(), palo: "corazones".to_string(), valor: 7},
        Carta { numero: "8".to_string(), palo: "corazones".to_string(), valor: 8},
        Carta { numero: "9".to_string(), palo: "corazones".to_string(), valor: 9},
        Carta { numero: "10".to_string(), palo: "corazones".to_string(), valor: 10},
        Carta { numero: "J".to_string(), palo: "corazones".to_string(), valor: 11},
        Carta { numero: "Q".to_string(), palo: "corazones".to_string(), valor: 12},
        Carta { numero: "K".to_string(), palo: "corazones".to_string(), valor: 13},

        Carta { numero: "As".to_string(), palo: "treboles".to_string(), valor: 100},
        Carta { numero: "2".to_string(), palo: "treboles".to_string(), valor: 2},
        Carta { numero: "3".to_string(), palo: "treboles".to_string(), valor: 3},
        Carta { numero: "4".to_string(), palo: "treboles".to_string(), valor: 4},
        Carta { numero: "5".to_string(), palo: "treboles".to_string(), valor: 5},
        Carta { numero: "6".to_string(), palo: "treboles".to_string(), valor: 6},
        Carta { numero: "7".to_string(), palo: "treboles".to_string(), valor: 7},
        Carta { numero: "8".to_string(), palo: "treboles".to_string(), valor: 8},
        Carta { numero: "9".to_string(), palo: "treboles".to_string(), valor: 9},
        Carta { numero: "10".to_string(), palo: "treboles".to_string(), valor: 10},
        Carta { numero: "J".to_string(), palo: "treboles".to_string(), valor: 11},
        Carta { numero: "Q".to_string(), palo: "treboles".to_string(), valor: 12},
        Carta { numero: "K".to_string(), palo: "treboles".to_string(), valor: 13},

    ];

    Mazo {
        cartas
    }
}