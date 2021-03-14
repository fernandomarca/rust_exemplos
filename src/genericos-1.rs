fn main() {
    fn maior_g<T: PartialOrd + Copy>(lista: &[T]) -> T {
        let mut maior = list[0];
        for &item in list.iter() {
            if item > maior {
                maior = item;
            }
        }
        maior
    }

    fn maior(list: &[i32]) -> i32 {
        let mut maior = list[0];
        for &item in list.iter() {
            if item > maior {
                maior = item;
            }
        }
        maior
    }

    let lista_numero = vec![34, 50, 25, 100, 65];
    let resultado = maior(&lista_numero);
    println!("O maior número é {}", resultado);
    let lista_numero = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let resultado = maior(&lista_numero);
    println!("O maior número é {}", resultado);

    //genéricos com trait

    use std::fmt::Display;

    struct Par<T> {
        x: T,
        y: T,
    }

    impl<T> Par<T> {
        fn novo(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Par<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("O maior membro é x = {}", self.x);
            } else {
                println!("O maior membro é y = {}", self.y);
            }
        }
    }
}
