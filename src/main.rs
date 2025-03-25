use dns_parser;
use std::{net, time::Duration};

mod app;

fn main() {
    let container = app::ParamsContainer::new();

    // TODO: реально использовать все это дерьмо с билдером днс квари.

    // Create a reference and save it to a variable builder.
    let mut builder = dns_parser::Builder::new_query(rand::random::<u16>(), false);

    // Here we take a reference to a builder and use it to add a question.'
    // reference is taken due to add_question requirement of &mut self.
    builder.add_question(
        "facebook.com",
        false,
        dns_parser::QueryType::A,
        dns_parser::QueryClass::IN,
    );

    // move refence. builder is dropped.
    let query_as_bytes: Vec<u8> = builder.build().unwrap();

    // Let's try to bind a socket and connect to external address.
    match net::UdpSocket::bind(format!("127.0.0.1:{}", container.udp_socket_port)) {
        Ok(socket) => {
            socket
                .set_read_timeout(Some(Duration::from_secs(3)))
                .expect("failed to set read timeout.");

            println!("using {}", socket.local_addr().unwrap());

            socket
                .set_nonblocking(false)
                .expect("failed to set nonblocking");

            match socket.connect(&container.dns_server_addr) {
                Ok(_) => {
                    println!("successfully connected to a given address.");
                }
                Err(e) => panic!("failed to connect to given address: {e}"),
            }

            // Fails. failed to send packets to DNS server: Can't assign requested address (os error 49)
            // TODO: figure out why
            socket
                .send(&query_as_bytes)
                .expect("failed to send packets to DNS server");

            // Creating a buffer.
            let mut buf: Vec<u8> = vec![0; 1024];
            socket
                .recv(&mut buf)
                .expect("failed to receive packets from DNS server");
        }
        Err(e) => eprintln!("{e}"),
    }
}
