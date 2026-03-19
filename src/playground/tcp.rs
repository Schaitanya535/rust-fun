use std::net::{Ipv4Addr, TcpListener};

use anyhow::Result;

fn tcp_server() -> Result<TcpListener> {
    let addr = Ipv4Addr::new(127, 0, 0, 1);
    let tcp_listener = TcpListener::bind(addr.to_string()).unwrap();
    Ok(tcp_listener)
}

pub fn main() {
    let server = tcp_server();
    match server {
        Ok(server) => {}
        Err(e) => {
            println!("{}", e)
        }
    }
}
