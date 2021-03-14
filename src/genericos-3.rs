fn main() {
    pub trait Resumir {
        fn resumo_autor(&self) -> String;
        fn resumo(&self) -> String {
            format!("(Leia mais de {}...)", self.resumo_autor())
        }
    }

    pub struct ArtigoDeNoticia {
        pub titulo: String,
        pub local: String,
        pub autor: String,
        pub conteudo: String,
    }
    impl Resumir for ArtigoDeNoticia {
        fn resumo(&self) -> String {
            format!("{}, by {} ({})", self.titulo, self.autor, self.local)
        }
    }
    pub struct Tweet {
        pub nomeusuario: String,
        pub conteudo: String,
        pub resposta: bool,
        pub retweet: bool,
    }
    impl Resumir for Tweet {
        fn resumo(&self) -> String {
            format!("{}: {}", self.nomeusuario, self.conteudo)
        }
        fn resumo_autor(&self) -> String {
            format!("@{}", self.nomeusuario)
        }
    }

    pub fn notificar<T: Resumir>(item: T) {
        println!("Notícias de última hora! {}", item.resumo());
    }

    //implementa funcao com trait - interface

    fn alguma_funcao<T: Mostrar + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

    fn alguma_funcao2<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
    }
}
