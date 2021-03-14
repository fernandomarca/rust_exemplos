fn main() {
    fn maior_g<T>(lista: &[T]) -> T {}
    struct Ponto<T> {
        x: T,
        y: T,
    }
    struct Ponto2<T, U> {
        x: T,
        y: U,
    }
    let inteiro = Ponto { x: 5, y: 10 };
    let float = Ponto2 { x: 1.0, y: 4.0 };

    //genericos para enums
    enum Option<T> {
        Some(T),
        None,
    }

    enum Resultado<T, E> {
        Ok(T),
        Err(E),
    }

    //generico pra struct e impl

    struct Ponto<T> {
        x: T,
        y: T,
    }
    impl<T> Ponto<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p = Ponto { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    //mistura
    struct Ponto<T, U> {
        x: T,
        y: U,
    }
    impl<T, U> Ponto<T, U> {
        fn mistura<V, W>(self, other: Ponto<V, W>) -> Ponto<T, W> {
            Ponto {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Ponto { x: 5, y: 10.4 };
    let p2 = Ponto { x: "Ola", y: 'c' };

    let p3 = p1.mistura(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
