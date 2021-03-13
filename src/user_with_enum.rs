fn main() {
    #[derive(Debug)]
    struct Color(i32, i64, i32);

    enum Sexo {
        M,
        F,
        None,
    }
    struct User {
        username: String,
        email: String,
        sign_in_count: u32,
        active: bool,
        color: Color,
        sexo: String,
    };
    impl User {
        fn count() -> u32 {
            55
        }
        fn sexo(x: Sexo) -> String {
            match x {
                Sexo::M => "M".to_string(),
                Sexo::F => "F".to_string(),
                Sexo::None => "nao declarado".to_string(),
            }
        }
        fn build_user(email: String, username: String) -> User {
            User {
                username,
                email,
                sign_in_count: User::count(),
                active: false,
                color: Color(1, 2, 3),
                sexo: User::sexo(Sexo::None),
            }
        }
    }
    let mut user3 = User::build_user("marcos@hotmail.com".to_string(), "Marcos".to_string());
    user3.sexo = User::sexo(Sexo::F);
    println!("usuário 3 {:#?}", user3.sexo);
}
