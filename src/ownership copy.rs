fn main() {
    let s = String::from("texto");  // s entra em escopo.

    toma_posse(s);                  // move o valor de s para dentro da função...
                                    // ... e ele não é mais válido aqui.

    let x = 5;                      // x entra em escopo.

    faz_uma_copia(x);               // x seria movido para dentro da função,
                                    // mas i32 é Copy, então está tudo bem em
                                    // usar x daqui para a frente.
    println!("{}",s);

} // Aqui, x sai de escopo, e depois s. Mas como o valor de s foi movido, nada
  // de especial acontece.

fn toma_posse(uma_string: String) { // uma_string entra em escopo.
    println!("{}", uma_string);
} // Aqui, uma_string sai de escopo, e o método `drop` é chamado. A memória que
  // guarda seus dados é liberada.

fn faz_uma_copia(um_inteiro: i32) { // um_inteiro entra em escopo.
    println!("{}", um_inteiro);
} // Aqui, um_inteiro sai de escopo. Nada de especial acontece.
