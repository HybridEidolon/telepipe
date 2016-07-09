#[derive(Clone, RustcDecodable, Debug)]
pub struct Config {
    pub bind_addr: String,
    pub server_addr: String,
    pub listen_ports: Vec<(u16, u16)>,
    pub use_dns: bool
}
