fn main() {
    #[derive(Debug)]
    struct Ipv4Addr {
        ip: (u8, u8, u8, u8),
    }

    #[derive(Debug)]
    struct Ipv6Addr {
        ip: String,
    }

    #[derive(Debug)]
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    let adrr2 = IpAddr::V4(Ipv4Addr { ip: (127, 0, 0, 1) });

    enum VersaoIp {
        v4(u8, u8, u8, u8),
        v6,
    }

    struct EnderecoIp {
        versao: VersaoIp,
        endereco: String,
    }

    let quatro = VersaoIp::v4;
    let seis = VersaoIp::v6;

    fn rotear(versao_ip: VersaoIp) {}

    let local = EnderecoIp {
        versao: VersaoIp::v4(127, 0, 0, 1),
        endereco: String::from("127.0.0.1"),
    };
    let loopback = EnderecoIp {
        versao: VersaoIp::v6,
        endereco: String::from("127.0.0.1"),
    };

    #[derive(Debug)]
    enum Mensagem {
        Sair,
        Mover { x: i32, y: i32 },
        Escrever(String),
        MudaCor(i32, i32, i32),
    }

    impl Mensagem {
        fn invocar(&self) {}
    }

    let msg1 = Mensagem::Escrever(String::from("Fernando"));
}
