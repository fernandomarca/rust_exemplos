use std::io::ErrorKind;
use std::{fs::File, io::Read};
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Tentou criar um arquivo e houve um problema: {:?}", e)
            }
        },
        Err(error) => {
            panic!("Houve um problema ao abrir o arquivo: {:?}", error)
        }
    };
}
