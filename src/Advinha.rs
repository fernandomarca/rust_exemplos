use std::io;
extern crate rand;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Advinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1..101);

    // println!("O número secreto é: {}", numero_secreto);

    loop {
        println!("Digite o seu palpite");

        let mut palpite = String::new();

        match io::stdin().read_line(&mut palpite) {
            Ok(_) => {
                // println!("{} bytes read", _);
                println!("{resposta}", resposta = palpite);
            }
            Err(error) => println!("error: {}", error),
        };

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if palpite < 1 || palpite > 100 {
            println!("O número secreto vai estar entre 1 e 100.");
            continue;
        }

        println!("você disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("muito baixo!"),
            Ordering::Greater => println!("muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
