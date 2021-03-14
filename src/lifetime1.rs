fn main() {
    let string1 = String::from("abcd");
    let resultado;
    {
        let string2 = String::from("xyz");
        resultado = maior(string1.as_str(), string2.as_str());
    }
    println!("A string mais longa é {}", resultado);

    fn maior<'a>(x: &'a str, y: &'b str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
