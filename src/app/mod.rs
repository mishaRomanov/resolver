use dotenv;
use std::net;

// Environmentals is a container for all env variables.
struct Environmentals {
    // Default if 8.8.8.8 - Public Google DNS server.
    dns_server: String,
}
impl Environmentals {
    fn parse() -> Environmentals {
        // Parse all envs from .env to actual environment.
        // Might create some more variables later so it should be useful in the future.
        dotenv::dotenv().ok();

        // Parsing ip addr of DNS server from environment.
        match std::env::var("DNS_SERVER") {
            Ok(server) => Environmentals { dns_server: server },
            Err(_e) => {
                eprintln!(
                    "failed to parse dns_server env variable. setting default one. 8.8.8.8...."
                );
                Environmentals {
                    dns_server: "8.8.8.8".to_string(),
                }
            }
        }
    }
}

// I dunno yet just decided to group information like that for now.
#[derive(Debug, Clone, Copy)]
pub struct ParamsContainer {
    pub v4_socket_addr: net::SocketAddrV4,
}

impl ParamsContainer {
    pub fn new() -> ParamsContainer {
        let envs = Environmentals::parse();

        let ip_addr = envs
            .dns_server
            .parse::<net::Ipv4Addr>()
            .expect("failed to parse DNS's IP addr");
        ParamsContainer {
            v4_socket_addr: net::SocketAddrV4::new(ip_addr, 53),
        }
    }
}
