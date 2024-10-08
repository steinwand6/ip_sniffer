use std::{env, net::IpAddr};

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

    let (tx, mut rx) = tokio::sync::mpsc::channel(200);
    for port in 1..=MAX {
        let tx = tx.clone();
        tokio::spawn(async move {
            let res = check_port(ip_addr, port).await;
            tx.send(res).await.unwrap();
        });
    }
    drop(tx);

    while let Some(res) = rx.recv().await {
        match res {
            None => (),
            Some(port) => println!("{:?} is open", port),
        }
    }
}

async fn check_port(ip: IpAddr, port: u16) -> Option<u16> {
    match TcpStream::connect((ip, port)).await {
        Ok(_) => Some(port),
        _ => None,
    }
}
