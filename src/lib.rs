pub trait Draw {
    fn desenhar(&self);
}

pub struct Janela {
    pub componentes: Vec<Box<Draw>>,
}

impl Janela {
    pub fn executar(&self) {
        for component in self.componentes.iter() {
            component.desenhar();
        }
    }
}

pub struct Button {
    pub largura: u32,
    pub altura: u32,
    pub label: String,
}

impl Draw for Button {
    fn desenhar(&self) {
        // Código para realmente desenhar um botão
    }
}

struct SelectBox {
    largura: u32,
    altura: u32,
    opcoes: Vec<String>,
}

impl Draw for SelectBox {
    fn desenhar(&self) {
        // Código para realmente desenhar um select box
    }
}
