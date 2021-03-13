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
        fn build_user(email: String, username: String) -> User {
            User {
                username,
                email,
                sign_in_count: User::count(),
                active: false,
                color: Color(1, 2, 3),
                sexo: Option<String>,
            }
        }
    }

    fn build_user(email: String, username: String) -> User {
        let user = User {
            username,
            email,
            sign_in_count: 11111,
            active: true,
            color: Color(0, 0, 0),
        };
        user
    }

    //let user1 = new User(p1,p2,p3,p4);
    let mut user1: User = build_user("ex@hotmail.com".to_string(), "Fernando".to_string());

    user1.active = false;

    let mut user2 = User {
        email: "ex2@hotmail.com".to_string(),
        active: true,
        sign_in_count: 2,
        username: "Fernando".to_string(),
        ..user1
    };

    println!("{:?}", user2);

    let user3 = User::build_user("marcos@hotmail.com".to_string(), "Marcos".to_string());

    println!("usuario 3 {:#?}", user3.sign_in_count);
}
