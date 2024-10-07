use std::{
    env,
    net::{IpAddr, TcpStream},
};

const MAX: u16 = 65535;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ip_sniffer <ip>");
        return;
    }

    let ip_addr: IpAddr = args[1]
        .parse()
        .expect("Unable to parse param as IP address");

    for port in 0..MAX {
        match TcpStream::connect((ip_addr, port)) {
            Ok(_) => println!("{} is open!", port),
            Err(_) => (),
        }
    }
}
