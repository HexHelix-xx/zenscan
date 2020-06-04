extern crate clap;

use clap::{App, Arg};
use std::thread;
use std::sync::mpsc::{Sender, channel};
use std::time::Duration;
use std::net::{TcpStream, IpAddr, Ipv4Addr, SocketAddr};
use std::ops::AddAssign;
const _NANO: u32 = 500_000_000;
const SECOND: u32 = 5;
fn scanner(gabriel: Sender<u16>, range: Vec<u16>, ip_address: Ipv4Addr) {

    for port_number in range {
        let socket = SocketAddr::new(IpAddr::from(ip_address), port_number);

        if TcpStream::connect_timeout(&socket, Duration::new(SECOND.into(), _NANO)).is_ok() {
            println!("\x1b[34m[+]\x1b[m found the open port => {:?}", port_number);
            gabriel.send(port_number).unwrap();
        }
    }   
}

fn main() {
    let cli = App::new("Zen-Scan")
        .version("0.1.0")
        .author("Nirna Kayanovsky <nirnakayanovsky@gmail.com>")
        .about("This program is port scanner to target server. not' normal port scanning to do and destructive specification changes may occur. This program is test stage ")
        .arg(
            Arg::with_name("zenscan <IpAddress>")
            .help("the address to scan")
            .required(true)
        ).get_matches();

    let ip = cli.value_of("zenscan <IpAddress>").unwrap_or("127.0.0.1");
    let thread: usize = 65535;
    let ip_address = ip.parse::<Ipv4Addr>().expect("\x1b[31m[-]\x1b[m CAN NOT PARSE YOUR INPUT INTO IPV4ADDR!");

    let (gabriel, diva_mater) = channel::<u16>();
    let mut open_ports = vec![];
    let socket_ports: Vec<u16> = (1..=65535).collect();
    let chunk_count: usize = 65535 / thread;
        
    println!("\x1b[34m[+]\x1b[m start scanning... by {}s", SECOND);

        let mut dispatched_threads = 0;

        for chunkblock in socket_ports.chunks(chunk_count) {
            let chunkblock = chunkblock.to_owned();
            let gabriel: Sender<u16> =    gabriel.clone();

            AddAssign::add_assign(&mut dispatched_threads, 1);

            thread::spawn(move || {
                scanner(gabriel, chunkblock, ip_address);
            });
        }
        drop(gabriel);

        for ports in diva_mater {
            open_ports.push(ports);
        }

        let open_iter = open_ports.iter();
    println!("----------------------open ports----------------------");
    println!("PORTS");        
            for target_ports in open_iter {
                println!("{}", target_ports);
            }
    println!("--------------------------end-------------------------");
}
