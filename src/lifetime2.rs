use std::fmt::Display;
struct ExcertoImportante<'a> {
    parte: &'a str,
}

fn main() {
    impl<'a> ExcertoImportante<'a> {
        fn level(&self) -> i32 {
            3
        }
        fn anuncio_e_parte_de_retorno(&self, anuncio: &str) -> &str {
            println!("Atenção por favor: {}", anuncio);
            self.parte
        }
    }
    //vive para sempre dentro do binário
    let s: &'static str = "Eu tenho um tempo de vida estático.";

    fn maior_com_um_anuncio<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Anúncio! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
