use dns_parser;
use std::{net, time::Duration};

mod app;
mod flags;

fn main() {
    let flags = flags::Flags::new();
    let container = app::ParamsContainer::new();

    // Create a reference and save it to a variable builder.
    let mut builder = dns_parser::Builder::new_query(rand::random::<u16>(), false);
    // Here we take a reference to a builder and use it to add a question.'
    // reference is taken due to add_question requirement of &mut self.
    builder.add_question(
        &flags.domain,
        false,
        dns_parser::QueryType::A,
        dns_parser::QueryClass::IN,
    );
    // move refence. builder is dropped.
    let query_as_bytes: Vec<u8> = builder.build().expect("failed to build query");

    // Let's try to bind a socket and connect to external address.
    match net::UdpSocket::bind(format!("0.0.0.0:{}", container.local_udp_socket_port)) {
        Ok(socket) => {
            socket
                .set_read_timeout(Some(Duration::from_secs(3)))
                .expect("failed to set read timeout.");

            println!("using {} to bind UDP socket", socket.local_addr().unwrap());

            socket
                .set_nonblocking(false)
                .expect("failed to set nonblocking");

            println!("asking {} for {}...", container.dns_server_addr,flags.domain);
            socket
                .send_to(&query_as_bytes, container.dns_server_addr)
                .expect("failed to send packets to DNS server");

            // Creating a buffer.
            let mut buf: Vec<u8> = vec![0; 512];
            socket
                .recv(&mut buf)
                .expect("failed to receive packets from DNS server");

            // Reading response.
            let response = dns_parser::Packet::parse(&buf).unwrap();

            println!("answer:");
            // Iterate over answers from DNS server.
            response.answers.iter().for_each(|r| 
            // We check the preferable ip type given by user.
                match flags.ip_type {
                flags::IpType::V4 => match r.data {
                    dns_parser::RData::A(a) => {
                        println!("IPv4: {}", a.0.to_string());
                    }
                    _ => println!("unknown"),
                },
                flags::IpType::V6 => match r.data {
                    dns_parser::RData::AAAA(a) => {
                        println!("IPv6: {}", a.0.to_string());
                    }
                    _ => println!("unknown"),
                },
            });
        }
        Err(e) => eprintln!("{e}"),
    }
}
