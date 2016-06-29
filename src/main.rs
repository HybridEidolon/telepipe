extern crate byteorder;
extern crate encoding;
#[macro_use] extern crate log;
extern crate env_logger;

pub mod serial;
pub mod msg;
pub mod crypto;

fn main() {
    println!("Hello, world!");
}