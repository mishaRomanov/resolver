use dotenv;
use std::net;

// Environmentals is a container for all env variables.
struct Environmentals {
    // Default if 8.8.8.8 - Public Google DNS server.
    dns_server: String,
    // Default if 5050.
    udp_socket_port: u16,
}
impl Environmentals {
    fn parse() -> Environmentals {
        // Parse all envs from .env to actual environment.
        // Might create some more variables later so it should be useful in the future.
        dotenv::dotenv().ok();

        let dns_server: String = std::env::var("DNS_SERVER").unwrap_or("8.8.8.8".to_string());
        let udp_socket_port: u16 = std::env::var("UDP_SOCKET_PORT")
            .unwrap_or("5050".to_string())
            .parse()
            .unwrap();

        Environmentals {
            dns_server,
            udp_socket_port,
        }
    }
}

// I dunno yet just decided to group information like that for now.
#[derive(Debug, Clone, Copy)]
pub struct ParamsContainer {
    pub dns_server_addr: net::SocketAddrV4,
    pub local_udp_socket_port: u16,
}

impl ParamsContainer {
    pub fn new() -> ParamsContainer {
        let envs = Environmentals::parse();

        let ip_addr = envs
            .dns_server
            .parse::<net::Ipv4Addr>()
            .expect("failed to parse DNS's IP addr");
        ParamsContainer {
            dns_server_addr: net::SocketAddrV4::new(ip_addr, 53),
            local_udp_socket_port: envs.udp_socket_port,
        }
    }
}
