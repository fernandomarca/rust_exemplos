extern crate gui;
use gui::Draw;
use gui::{Button, Janela};

fn main() {
    let screen = Janela {
        componentes: vec![
            Box::new(SelectBox {
                largura: 75,
                altura: 10,
                opcoes: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                largura: 50,
                altura: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.executar();
}
