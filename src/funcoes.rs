fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn _outra_funcao() -> i32 {
    let mut x: i32 = 3;
    x = x + 1;
    x
}

fn main() {
    println!("O valor de y é: {}", _outra_funcao());

    let rect1 = (30, 50);

    println!("a área do retângulo é: {}", area(rect1));
}
