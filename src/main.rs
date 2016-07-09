extern crate byteorder;
extern crate encoding;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate resolve;
extern crate rand;

pub mod serial;
pub mod msg;
pub mod crypto;
pub mod client;
pub mod session;

use std::net::Ipv4Addr;
use std::thread;

use resolve::{DnsSocket, MESSAGE_LIMIT, Message as DnsMsg, Resource, RecordType, Class};
use resolve::record;
use resolve::message::Qr;

use session::spawn_new_session;

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

fn main() {
    use std::str::FromStr;
    use std::net::SocketAddrV4;

    env_logger::init().unwrap();

    info!("Telepipe");
    info!("Waiting for connections...");

    let sock = DnsSocket::bind(SocketAddrV4::from_str("192.168.150.1:53").unwrap()).unwrap();
    thread::spawn(move || {
        dns_thread(sock);
    });

    spawn_new_session("192.168.150.1:9103", "74.59.188.106:9103", false);
    spawn_new_session("192.168.150.1:9003", "74.59.188.106:9003", false);
    spawn_new_session("192.168.150.1:9203", "74.59.188.106:9203", false);
    spawn_new_session("192.168.150.1:9002", "74.59.188.106:9002", false);

    spawn_new_session("192.168.150.1:9100", "74.59.188.106:9100", false);
    spawn_new_session("192.168.150.1:9001", "74.59.188.106:9001", false);

    loop {
        use std::time::Duration;
        thread::sleep(Duration::from_millis(1000));
    }
}
