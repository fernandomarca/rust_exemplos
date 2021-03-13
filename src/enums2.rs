fn main() {
    #[derive(Debug)]
    enum Estado {
        Alabama,
        Alaska,
        Texas,
        Lasvegas,
    }
    enum Moeda {
        Penny,
        Nickel,
        Dime,
        Quarter(Estado),
    }

    fn valor_em_cents(moeda: Moeda) -> u32 {
        match moeda {
            Moeda::Penny => 1,
            Moeda::Nickel => 5,
            Moeda::Dime => 10,
            Moeda::Quarter(estado) => {
                println!("{:?}", estado);
                25
            }
        }
    }

    let m = Moeda::Dime;

    println!("{}", valor_em_cents(m));
}
