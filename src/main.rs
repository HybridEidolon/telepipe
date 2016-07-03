extern crate byteorder;
extern crate encoding;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate resolve;

pub mod serial;
pub mod msg;
pub mod crypto;
pub mod client;

use std::net::{TcpListener, Ipv4Addr};
use std::thread;

use resolve::{DnsSocket, MESSAGE_LIMIT, Message as DnsMsg, Resource, RecordType, Class};
use resolve::record;
use resolve::message::Qr;

use client::Session;

fn init_log_config() {
    let r = std::env::var("RUST_LOG");
    match r {
        Ok(ref s) if s != "" => (),
        _ => std::env::set_var("RUST_LOG", "debug")
    }
}

fn dns_thread(socket: DnsSocket) {
    info!("DNS thread live");
    let mut buf = vec![0; MESSAGE_LIMIT];
    loop {
        match socket.recv_from(&mut buf[..]) {
            Ok((m, addr)) => {
                info!("DNS request from {:?}, responding with stock message", addr);
                let mut mm = DnsMsg::new();
                mm.header.authoritative = true;
                mm.header.id = m.header.id;
                mm.header.qr = Qr::Response;
                mm.header;
                let mut resource = Resource::new("gc01.st-pso.games.sega.net".to_string(), RecordType::A, Class::Internet, 59);
                resource.write_rdata(&record::A { address: Ipv4Addr::new(192, 168, 150, 1) }).unwrap();
                mm.answer = vec![resource.clone()];
                mm.authority = vec![resource.clone()];
                mm.header.recursion_available = false;
                if let Err(_) = socket.send_message(&mm, addr) {
                    error!("Error occurred sending response in dns thread");
                }
            },
            Err(_) => {
                error!("Error occurred in dns thread");
                break;
            }
        }
    }
}

fn tcp_listener_thread(listener: TcpListener) {
    let port = listener.local_addr().unwrap().port();
    info!("TCP listener thread live on port {:?}", port);

    for s in listener.incoming() {
        match s {
            Ok(s) => {
                use std::str::FromStr;
                use std::net::SocketAddr;

                info!("Connected {:?} on port {:?}.", s.peer_addr(), port);
                let addr = format!("74.59.188.106:{}", port);
                let addr = <SocketAddr as FromStr>::from_str(&addr).unwrap();
                let session: Session = Session::new(s, addr).unwrap();
                thread::spawn(move|| {
                    use client::Run;
                    info!("Session spawned.");
                    session.run();
                });
            }
            Err(_) => {
                error!("Error in TCP listener");
                break;
            }
        }
    }
}

fn main() {
    use std::str::FromStr;
    use std::net::SocketAddrV4;

    // TODO replace this with user configurable via config file
    init_log_config();
    env_logger::init().unwrap();

    info!("Telepipe");
    info!("Waiting for connections...");

    let sock = DnsSocket::bind(SocketAddrV4::from_str("192.168.150.1:53").unwrap()).unwrap();
    thread::spawn(move || {
        dns_thread(sock);
    });

    let listener = TcpListener::bind(SocketAddrV4::from_str("192.168.150.1:9103").unwrap()).unwrap();

    thread::spawn(move || {
        tcp_listener_thread(listener);
    });

    let listener = TcpListener::bind(SocketAddrV4::from_str("192.168.150.1:9003").unwrap()).unwrap();

    thread::spawn(move || {
        tcp_listener_thread(listener);
    });

    let listener = TcpListener::bind(SocketAddrV4::from_str("192.168.150.1:9203").unwrap()).unwrap();

    thread::spawn(move || {
        tcp_listener_thread(listener);
    });

    let listener = TcpListener::bind(SocketAddrV4::from_str("192.168.150.1:9002").unwrap()).unwrap();

    thread::spawn(move || {
        tcp_listener_thread(listener);
    });

    // Episodes 1 & 2 ports
    let listener = TcpListener::bind(SocketAddrV4::from_str("192.168.150.1:9100").unwrap()).unwrap();

    thread::spawn(move || {
        tcp_listener_thread(listener);
    });

    let listener = TcpListener::bind(SocketAddrV4::from_str("192.168.150.1:9001").unwrap()).unwrap();

    thread::spawn(move || {
        tcp_listener_thread(listener);
    });

    loop {
        use std::time::Duration;
        thread::sleep(Duration::from_millis(1000));
    }
}
