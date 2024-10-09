use std::{env, net::IpAddr};

use futures::{stream, StreamExt};
use tokio::net::TcpStream;

const MAX: u16 = 65535;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ip_sniffer <ip>");
        return;
    }

    let ip_addr: IpAddr = args[1]
        .parse()
        .expect("Unable to parse param as IP address");

    stream::iter(1..=MAX)
        .for_each_concurrent(200, |port| async move {
            if is_open_port(ip_addr, port).await {
                println!("{port} is open");
            }
        })
        .await;
}

async fn is_open_port(ip: IpAddr, port: u16) -> bool {
    TcpStream::connect((ip, port)).await.is_ok()
}
