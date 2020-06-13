extern crate clap;
extern crate rayon;
extern crate pnet;
extern crate dns_lookup;
extern crate better_panic;
use clap::{App, Arg};
use std::thread;
use std::sync::mpsc::{Sender, channel};
use std::time::Duration;
use std::net::{TcpStream, IpAddr, Ipv4Addr, SocketAddr};
use dns_lookup::lookup_host;

const VERSION_NUMBER: &str = "0.1.3";
const HELP: &str = "
        Zen-Scan 0.1.3
        Nirna Kayanovsky <nirnakayanovsky@gmail.com>

        This program is port scanner to target server.
        not' normal port scanning to do and destructive specification changes may occur.
        This program is test stage.

        USAGE:
            zenscan <options> 
                  
        Options:
        -h, --help            Display this massage
        -V, --version         Display version info
        -i <IPAddress>        The address to scan
        --host <Hostname>     The host scan, trans as IP
";
const _NANO: u32 = 500_000_000;
const SECOND: u32 = 5;

//Scanner body
fn scanner(alpha_group: Sender<u16>, range: Vec<u16>, network_node: IpAddr) {

    for port_number in range {
        let socket = SocketAddr::new(IpAddr::from(network_node), port_number);

        if TcpStream::connect_timeout(&socket, Duration::new(SECOND.into(), _NANO)).is_ok() {
            println!("\x1b[34m[+]\x1b[m found the open port => {:?}", port_number);
            alpha_group.send(port_number).unwrap();
        }
    }   
}

fn main() {
    better_panic::install();
    let cli = App::new("Zen-Scan")
        .help(HELP)
        .arg(
            Arg::with_name("ipaddress")
            .short("i")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("hostname")
            .long("host")
            .takes_value(true)
        )
        .get_matches();
   
    let mut ip = ""; let mut host = ""; 
    let mut parse_ip = IpAddr::V4(Ipv4Addr::new(0,0,0,0));
    let mut perse_host = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    
    //Parse the IP Address 
    if cli.value_of("ipaddress").is_none() && cli.value_of("hostname").is_none() {
        println!("{}", HELP);
        std::process::exit(1);
    } else if cli.value_of("ipaddress").is_some() || cli.value_of("hostname").is_none() {
        ip = cli.value_of("ipaddress").unwrap();  
        parse_ip = ip.parse::<IpAddr>().unwrap();
    } else if cli.value_of("ipaddress").is_none() || cli.value_of("hostname").is_some() {     
        host = cli.value_of("hostname").unwrap();
        let hostname: Vec<std::net::IpAddr> = lookup_host(host).expect("\x1b[31m[-]\x1b[m CAN NOT PARSE YOUR INPUT INTO HOST!");
        let host_to_ip = hostname[0].clone();
        perse_host = host_to_ip;
    }


    let thread: usize = 65535;
    let mut network_node = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    
    if cli.value_of("ipaddress").is_some() {
        if ip.len() <= 3 {
            eprintln!("\x1b[31m[-]\x1b[m CAN NOT PARSE YOUR INPUT INTO IPADDRESS!");
            std::process::exit(1);
        } else {
            network_node = parse_ip;
        }
    } else if cli.value_of("hostname").is_some() {
         if perse_host.is_unspecified() {
            eprintln!("\x1b[31m[-]\x1b[m CAN NOT PARSE YOUR INPUT INTO HOST!");
            std::process::exit(1);
        } else { 
            network_node = perse_host;
        }
    }
    
    let (alpha_group, chq) = channel::<u16>();
    let mut open_ports = vec![];
    let socket_ports: Vec<u16> = (1..=65535).collect();
    let chunk_count: usize = 65535 / thread;
    
    // flow of control 
    println!("\x1b[34m[+]\x1b[m start scanning... by {}s. zenscan version for {}", SECOND, VERSION_NUMBER);

        for chunkblock in socket_ports.chunks(chunk_count) {
            let chunkblock = chunkblock.to_owned();
            let alpha_group: Sender<u16> = alpha_group.clone();

            thread::spawn(move || {
                scanner(alpha_group, chunkblock, network_node);
            });
        }
        drop(alpha_group);

        for ports in chq {
            open_ports.push(ports);
        }

    println!("----------------------open ports----------------------");
    println!("PORTS");        
            for target_ports in open_ports {
                println!("{}", target_ports);
            }
    println!("--------------------------end-------------------------");
}
