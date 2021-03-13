fn main() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        length: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.length
        }
        fn area2(self) -> (u32, Rectangle) {
            (self.length * self.width, self)
        }
    }

    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };

    println!("a área do retângulo é: {}", rect1.area());

    let mut rect2 = Rectangle {
        width: 30,
        length: 40,
    };

    let (area, mut rect2) = rect2.area2();

    println!("a área do retângulo é:{:#?}", rect2);
}
