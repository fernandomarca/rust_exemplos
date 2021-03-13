fn est() {
    let numero = " ";

    if numero.is_empty() {
        println!("condição era verdadeira");
    } else {
        println!("condição era falsa");
    }
}

fn estr1(x: i32) -> i32 {
    let numero = x;

    let y = if numero <= 5 { 5 } else { 6 };

    y
}

fn main() {
    est();
    println!("{}", estr1(5));
}
