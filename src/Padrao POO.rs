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

    // exemplo
    struct ICreateCategoryDTO {
        name: String,
        description: String,
    }
    //desestruturar
    let p = ICreateCategoryDTO {
        name: String::from("FErnando"),
        description: String::from("description"),
    };
    let ICreateCategoryDTO { name, description } = p;

    pub struct Category {
        id: String,
        name: String,
        description: String,
    }
    //interface
    pub trait ICategoriesRepository {
        fn desenhar(&self);
        fn findByName(&self, name: String) -> Category;
        fn list(&self) -> [Category];
        fn create(ICreateCategoryDTO { name, description }: ICreateCategoryDTO) -> void;
    }
}
