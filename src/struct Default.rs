use std::default::Default;

fn main() {
    #[derive(Debug)]
    pub struct Sample {
        a: u32,
        b: u32,
        c: u32,
    }

    impl Default for Sample {
        fn default() -> Self {
            Sample { a: 2, b: 4, c: 6 }
        }
    }

    let s = Sample {
        c: 23,
        ..Sample::default()
    };
    println!("{:?}", s.c);
}
