pub struct Carta {
    pub numero: String,
    pub palo: String,
}


pub struct Mazo {
    pub cartas: std::vec::Vec::<Carta>,
}


pub fn nuevo() -> Mazo {

    let cartas = vec![
        Carta { numero: "As".to_string(), palo: "diamantes".to_string()},
        Carta { numero: "2".to_string(), palo: "diamantes".to_string()},
        Carta { numero: "3".to_string(), palo: "diamantes".to_string()},
        Carta { numero: "4".to_string(), palo: "diamantes".to_string()},
        Carta { numero: "5".to_string(), palo: "diamantes".to_string()},
        Carta { numero: "6".to_string(), palo: "diamantes".to_string()},
        Carta { numero: "7".to_string(), palo: "diamantes".to_string()},
        Carta { numero: "8".to_string(), palo: "diamantes".to_string()},
        Carta { numero: "9".to_string(), palo: "diamantes".to_string()},
        Carta { numero: "J".to_string(), palo: "diamantes".to_string()},
        Carta { numero: "Q".to_string(), palo: "diamantes".to_string()},
        Carta { numero: "K".to_string(), palo: "diamantes".to_string()},
        
        Carta { numero: "As".to_string(), palo: "picas".to_string()},
        Carta { numero: "2".to_string(), palo: "picas".to_string()},
        Carta { numero: "3".to_string(), palo: "picas".to_string()},
        Carta { numero: "4".to_string(), palo: "picas".to_string()},
        Carta { numero: "5".to_string(), palo: "picas".to_string()},
        Carta { numero: "6".to_string(), palo: "picas".to_string()},
        Carta { numero: "7".to_string(), palo: "picas".to_string()},
        Carta { numero: "8".to_string(), palo: "picas".to_string()},
        Carta { numero: "9".to_string(), palo: "picas".to_string()},
        Carta { numero: "J".to_string(), palo: "picas".to_string()},
        Carta { numero: "Q".to_string(), palo: "picas".to_string()},
        Carta { numero: "K".to_string(), palo: "picas".to_string()},

        Carta { numero: "As".to_string(), palo: "corazones".to_string()},
        Carta { numero: "2".to_string(), palo: "corazones".to_string()},
        Carta { numero: "3".to_string(), palo: "corazones".to_string()},
        Carta { numero: "4".to_string(), palo: "corazones".to_string()},
        Carta { numero: "5".to_string(), palo: "corazones".to_string()},
        Carta { numero: "6".to_string(), palo: "corazones".to_string()},
        Carta { numero: "7".to_string(), palo: "corazones".to_string()},
        Carta { numero: "8".to_string(), palo: "corazones".to_string()},
        Carta { numero: "9".to_string(), palo: "corazones".to_string()},
        Carta { numero: "J".to_string(), palo: "corazones".to_string()},
        Carta { numero: "Q".to_string(), palo: "corazones".to_string()},
        Carta { numero: "K".to_string(), palo: "corazones".to_string()},

        Carta { numero: "As".to_string(), palo: "treboles".to_string()},
        Carta { numero: "2".to_string(), palo: "treboles".to_string()},
        Carta { numero: "3".to_string(), palo: "treboles".to_string()},
        Carta { numero: "4".to_string(), palo: "treboles".to_string()},
        Carta { numero: "5".to_string(), palo: "treboles".to_string()},
        Carta { numero: "6".to_string(), palo: "treboles".to_string()},
        Carta { numero: "7".to_string(), palo: "treboles".to_string()},
        Carta { numero: "8".to_string(), palo: "treboles".to_string()},
        Carta { numero: "9".to_string(), palo: "treboles".to_string()},
        Carta { numero: "J".to_string(), palo: "treboles".to_string()},
        Carta { numero: "Q".to_string(), palo: "treboles".to_string()},
        Carta { numero: "K".to_string(), palo: "treboles".to_string()},

    ];

    Mazo {
        cartas
    }
}