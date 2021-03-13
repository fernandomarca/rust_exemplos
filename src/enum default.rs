use std::default::Default;

fn main() {
    #[derive(Debug, Default)]
    pub struct Sample {
        a: char,
        b: u32,
        c: u32,
    }

    enum Exemplo {
        M,
        F,
        None,
    }

    impl Default for Exemplo {
        fn default() -> Self {
            Exemplo::None
        }
    }

    // impl Default for Sample {
    //     fn default() -> Self {
    //         Sample { a: 2, b: 4, c: 6 }
    //     }
    // }

    let s = Sample {
        c: 23,
        ..Sample::default()
    };
    println!("{}", s.a);
}
