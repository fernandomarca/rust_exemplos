use std::ops::Deref;
fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let w = MeuBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *w);

    //ponteiro inteligente
    struct MeuBox<T>(T);

    impl<T> MeuBox<T> {
        fn new(x: T) -> MeuBox<T> {
            MeuBox(x)
        }
    }

    impl<T> Deref for MeuBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }
    //coerção de desreferencia
    let m = MeuBox::new(String::from("Rust"));
    ola(&m);
    //ola(&(*m)[..]); sem coerção

    fn ola(nome: &str) {
        println!("Olá, {}!", nome);
    }

    //trait drop
    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Destruindo CustomSmartPointer com dados `{}`!", self.data);
        }
    }
    let c = CustomSmartPointer {
        data: String::from("alocado primeiro"),
    };
    let d = CustomSmartPointer {
        data: String::from("alocado por último"),
    };
    println!("CustomSmartPointers criados.");
}
