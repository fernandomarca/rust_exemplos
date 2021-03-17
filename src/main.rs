extern crate blog;
use blog::Postagem;
fn main() {
    let mut post = Postagem::new();

    post.add_texto("Eu comi uma salada no almoço de hoje");
    assert_eq!("", post.conteudo());

    post.solicitar_revisao();
    assert_eq!("", post.conteudo());

    post.aprovar();
    assert_eq!("Eu comi uma salada no almoço de hoje", post.conteudo());
}
