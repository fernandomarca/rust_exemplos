

fn primeira_palavra(s: &str) -> &str {
  let bytes = s.as_bytes();
  println!("{:?}",bytes);

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}

fn main() {
  let mut s = String::from("texto longo");
  
  
  let palavra = primeira_palavra(&mut s[..]);
  println!("{}",palavra);

  s.clear(); // Erro!
  println!("{}",s);

}