fn main() {
  let s1 = String::from("texto");

  let (s1, tamanho) = calcula_tamanho(s1);

  println!("O tamanho de '{}' é {}.", s1, tamanho);
}

fn calcula_tamanho(s: String) -> (String, usize) {
  let tamanho = s.len(); // len() retorna o tamanho de uma String.

  (s, tamanho)
}