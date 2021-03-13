fn main() {
  let mut s1 = String::from("texto");

  let tamanho = calcula_tamanho(&mut s1);

  println!("O tamanho de '{}' é {}.", s1, tamanho);
}

fn calcula_tamanho(s: &mut String) -> usize {
  let tamanho = s.len();
  s.push_str(" longo"); // len() retorna o tamanho de uma String.
  tamanho
}