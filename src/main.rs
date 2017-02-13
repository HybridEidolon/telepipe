extern crate byteorder;
extern crate encoding;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate resolve;
extern crate rand;
extern crate toml;
extern crate rustc_serialize;
extern crate clap;

pub mod serial;
pub mod msg;
pub mod crypto;
pub mod client;
pub mod session;
pub mod config;
pub mod dns;
pub mod util;

use std::net::Ipv4Addr;
use std::thread;
use std::io::{self, Read};
use std::fs::File;

use clap::{App, Arg};
use resolve::DnsSocket;

use session::spawn_new_session;
use config::Config;

fn main() {
    env_logger::init().unwrap();

    let matches = App::new("Telepipe")
        .version("1.1.1")
        .author("Eidolon (@HybridEidolon)")
        .about("A proxy for Phantasy Star Online games for the GameCube.")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("PATH")
            .help("Sets the config file.")
            .default_value("telepipe.toml"))
        .get_matches();

    let config = matches.value_of("config").unwrap();

    debug!("Config file in use: {}", config);

    // Parse the config
    let mut contents = String::new();
    File::open(config).unwrap().read_to_string(&mut contents).unwrap();
    let config: Config = match toml::decode_str::<Config>(&contents) {
        Some(c) => c,
        None => {
            println!("The config file is incorrect. Please correct it.");
            return;
        }
    };

    if config.use_dns {
        use std::str::FromStr;

        let sock = match DnsSocket::bind((config.bind_addr.as_str(), 53)) {
            Ok(v) => v,
            Err(e) => {
                println!("An error binding the DNS server occurred: {}", e);
                if e.kind() == io::ErrorKind::AddrNotAvailable {
                    println!("If you are trying to play in Dolphin, it is likely that you have \
                              not initialized the TAP adapter yet. Please make sure to install \
                              and setup your TAP adapter and wait until the 'Connecting to DNS \
                              Server' prompt in-game before running this program.");
                }
                util::pause().unwrap();
                return;
            }
        };
        let proxy_server_addr = Ipv4Addr::from_str(config.proxy_server_addr.as_str()).unwrap();
        thread::spawn(move || { dns::dns_thread(sock, proxy_server_addr); });
    }

    for (local, server) in config.listen_ports {
        match spawn_new_session((config.bind_addr.as_str(), local),
                                (config.server_addr.as_str(), server),
                                config.proxy_server_addr.clone(),
                                config.bind_addr.clone(),
                                false) {
            Err(e) => {
                println!("An error occurred: {}", e);
                match e.kind() {
                    _ => println!("{:?}", e),
                }
                util::pause().unwrap();
                return;
            }
            _ => (),
        }
    }

    loop {
        use std::time::Duration;
        thread::sleep(Duration::from_millis(1000));
    }
}
