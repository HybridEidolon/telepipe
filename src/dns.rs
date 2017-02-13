use std::net::Ipv4Addr;

use resolve::{DnsSocket, MESSAGE_LIMIT, Message, Resource, RecordType, Class};
use resolve::record;
use resolve::message::Qr;

pub fn dns_thread(socket: DnsSocket, server_addr: Ipv4Addr) {
    info!("DNS thread live");
    let mut buf = vec![0; MESSAGE_LIMIT];
    loop {
        match socket.recv_from(&mut buf[..]) {
            Ok((m, addr)) => {
                info!("DNS request from {:?}, responding with stock message", addr);
                let mut mm = Message::new();
                mm.header.authoritative = true;
                mm.header.id = m.header.id;
                mm.header.qr = Qr::Response;
                mm.header;
                let mut resource = Resource::new("gc01.st-pso.games.sega.net".to_string(),
                                                 RecordType::A,
                                                 Class::Internet,
                                                 59);
                resource.write_rdata(&record::A { address: server_addr }).unwrap();
                mm.answer = vec![resource.clone()];
                mm.authority = vec![resource.clone()];
                mm.header.recursion_available = false;
                if let Err(_) = socket.send_message(&mm, addr) {
                    error!("Error occurred sending response in dns thread");
                }
            }
            Err(_) => {
                error!("Error occurred in dns thread");
                break;
            }
        }
    }
}
