#[derive(Clone, RustcDecodable, Debug)]
pub struct Config {
    pub bind_addr: String,
    pub server_addr: String,
    pub listen_ports: Vec<(u16, u16)>,
    pub use_dns: bool,
    pub proxy_server_addr: String,
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;

    use toml;

    use super::*;

    #[test]
    fn default_config_is_valid() {
        let mut contents = String::new();
        File::open("telepipe.toml").unwrap().read_to_string(&mut contents).unwrap();
        toml::decode_str::<Config>(&contents).unwrap();
    }
}
