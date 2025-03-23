extern crate clap;

use std::net;

mod app;

fn main() {
    let container = app::ParamsContainer::new();

    // TODO: реально использовать все это дерьмо с билдером днс квари.
    //
    // let query_id = rand::random::<u16>();
    // // А потом мы наверное делаем через tcp запрос с этим телом?
    // let query = dns_parser::Builder::new_query(query_id, false);

    // Let's try to bind a socket and connect to external address.
    match net::UdpSocket::bind("localhost:5555") {
        Ok(socket) => {
            socket
                .connect(&container.v4_socket_addr)
                .expect("failed to connect to given address");
        }
        Err(e) => eprintln!("{e}"),
    }
}
