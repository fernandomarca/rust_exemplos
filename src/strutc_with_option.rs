fn main() {
    #[derive(Debug)]
    struct Color(i32, i64, i32);
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u32,
        active: bool,
        color: Color,
        sexo: Option<String>,
    };
    impl User {
        fn count() -> u32 {
            55
        }
        fn sexo(x: Option<String>) -> Option<String> {
            match x {
                None => Some("não declarado".to_string()),
                Some(i) => Some(i),
            }
        }
        fn build_user(email: String, username: String) -> User {
            User {
                username,
                email,
                sign_in_count: User::count(),
                active: false,
                color: Color(1, 2, 3),
                sexo: User::sexo(None),
            }
        }
    }
    let mut user3 = User::build_user("marcos@hotmail.com".to_string(), "Marcos".to_string());
    user3.sexo = Some("M".to_string());
    println!("usuário 3 {:#?}", user3.sexo);
}
