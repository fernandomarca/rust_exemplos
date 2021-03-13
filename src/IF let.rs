fn main() {
    let algum_valor_u8: Option<u8> = Some(3);
    match algum_valor_u8 {
        Some(3) => println!("três"),
        _ => (),
    }
    if let Some(3) = algum_valor_u8 {
        println!("três");
    }
}
