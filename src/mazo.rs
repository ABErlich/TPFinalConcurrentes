#[derive(Clone)]
pub struct Carta {
    pub numero: Numero,
    pub palo: Palo
}

impl Carta {
    pub fn card_to_string(&self) -> String {
        let mut card: String = String::new();

        match self.numero {
            Numero::As =>  card.push_str("As"),
            Numero::Dos =>  card.push_str("2"),
            Numero::Tres => card.push_str("3"),
            Numero::Cuatro => card.push_str("4"),
            Numero::Cinco => card.push_str("5"),
            Numero::Seis => card.push_str("6"),
            Numero::Siete => card.push_str("7"),
            Numero::Ocho => card.push_str("8"),
            Numero::Nueve => card.push_str("9"),
            Numero::Diez => card.push_str("10"),
            Numero::J => card.push_str("J"),
            Numero::Q => card.push_str("Q"),
            Numero::K => card.push_str("K")
        };

        card.push_str(" de ");

        match self.palo {
            Palo::Diamantes => card.push_str("Diamantes"),
            Palo::Picas => card.push_str("Picas"),
            Palo::Corazones => card.push_str("Corazones"),
            Palo::Treboles => card.push_str("Treboles") 
        };

        return card

    }

    pub fn valor_carta(&self) -> u8 {
        match self.numero {
            Numero::As =>  return 100,
            Numero::Dos =>  return 2,
            Numero::Tres => return 3,
            Numero::Cuatro => return 4, 
            Numero::Cinco => return 5,
            Numero::Seis => return 6,
            Numero::Siete => return 7,
            Numero::Ocho => return 8,
            Numero::Nueve => return 9,
            Numero::Diez => return 10,
            Numero::J => return 11,
            Numero::Q => return 12,
            Numero::K => return 13
        };
    }

}
pub struct Mazo {
    pub cartas: std::vec::Vec::<Carta>,
}

#[derive(Clone)]
pub enum Palo {
    Diamantes,
    Picas,
    Corazones,
    Treboles
}

#[derive(Clone)]
pub enum Numero {
    As,
    Dos,
    Tres,
    Cuatro,
    Cinco,
    Seis,
    Siete,
    Ocho,
    Nueve,
    Diez,
    J,
    Q,
    K
}

pub fn nuevo() -> Mazo {

    let cartas = vec![
        Carta { numero: Numero::As, palo: Palo::Diamantes},
        Carta { numero: Numero::Dos, palo: Palo::Diamantes},
        Carta { numero: Numero::Tres, palo: Palo::Diamantes},
        Carta { numero: Numero::Cuatro, palo: Palo::Diamantes},
        Carta { numero: Numero::Cinco, palo: Palo::Diamantes},
        Carta { numero: Numero::Seis, palo: Palo::Diamantes},
        Carta { numero: Numero::Siete, palo: Palo::Diamantes},
        Carta { numero: Numero::Ocho, palo: Palo::Diamantes},
        Carta { numero: Numero::Nueve, palo: Palo::Diamantes},
        Carta { numero: Numero::Diez, palo: Palo::Diamantes},
        Carta { numero: Numero::J, palo: Palo::Diamantes},
        Carta { numero: Numero::Q, palo: Palo::Diamantes},
        Carta { numero: Numero::K, palo: Palo::Diamantes},
        
        Carta { numero: Numero::As, palo: Palo::Picas},
        Carta { numero: Numero::Dos, palo: Palo::Picas},
        Carta { numero: Numero::Tres, palo: Palo::Picas},
        Carta { numero: Numero::Cuatro, palo: Palo::Picas},
        Carta { numero: Numero::Cinco, palo: Palo::Picas},
        Carta { numero: Numero::Seis, palo: Palo::Picas},
        Carta { numero: Numero::Siete, palo: Palo::Picas},
        Carta { numero: Numero::Ocho, palo: Palo::Picas},
        Carta { numero: Numero::Nueve, palo: Palo::Picas},
        Carta { numero: Numero::Diez, palo: Palo::Picas},
        Carta { numero: Numero::J, palo: Palo::Picas},
        Carta { numero: Numero::Q, palo: Palo::Picas},
        Carta { numero: Numero::K, palo: Palo::Picas},

        Carta { numero: Numero::As, palo: Palo::Corazones},
        Carta { numero: Numero::Dos, palo: Palo::Corazones},
        Carta { numero: Numero::Tres, palo: Palo::Corazones},
        Carta { numero: Numero::Cuatro, palo: Palo::Corazones},
        Carta { numero: Numero::Cinco, palo: Palo::Corazones},
        Carta { numero: Numero::Seis, palo: Palo::Corazones},
        Carta { numero: Numero::Siete, palo: Palo::Corazones},
        Carta { numero: Numero::Ocho, palo: Palo::Corazones},
        Carta { numero: Numero::Nueve, palo: Palo::Corazones},
        Carta { numero: Numero::Diez, palo: Palo::Corazones},
        Carta { numero: Numero::J, palo: Palo::Corazones},
        Carta { numero: Numero::Q, palo: Palo::Corazones},
        Carta { numero: Numero::K, palo: Palo::Corazones},

        Carta { numero: Numero::As, palo: Palo::Treboles},
        Carta { numero: Numero::Dos, palo: Palo::Treboles},
        Carta { numero: Numero::Tres, palo: Palo::Treboles},
        Carta { numero: Numero::Cuatro, palo: Palo::Treboles},
        Carta { numero: Numero::Cinco, palo: Palo::Treboles},
        Carta { numero: Numero::Seis, palo: Palo::Treboles},
        Carta { numero: Numero::Siete, palo: Palo::Treboles},
        Carta { numero: Numero::Ocho, palo: Palo::Treboles},
        Carta { numero: Numero::Nueve, palo: Palo::Treboles},
        Carta { numero: Numero::Diez, palo: Palo::Treboles},
        Carta { numero: Numero::J, palo: Palo::Treboles},
        Carta { numero: Numero::Q, palo: Palo::Treboles},
        Carta { numero: Numero::K, palo: Palo::Treboles},

    ];

    Mazo {
        cartas
    }
}