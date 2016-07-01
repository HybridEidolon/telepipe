//! The structs representing a client's context.

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::net::{TcpStream, SocketAddr};
use std::thread;

use msg::*;

pub trait Run {
    fn run(self);
}

pub struct Session {
    session_receiver: Receiver<ProxyMsg>
}

pub struct Client {

}

pub struct Server {

}

enum ProxyMsg {
    FromServer(Msg),
    FromClient(Msg)
}

enum Source {
    Server,
    Client
}

impl Session {
    pub fn new(client: TcpStream, initial_server_addr: SocketAddr) -> Result<Session, String> {
        // Start new threads to process reads and writes, and use channels to handle incoming messages.
        // Use this session to handle messages from both the client and from the server.

        // Connect to the server.
        let server = TcpStream::connect(initial_server_addr).unwrap();

        let (session_sender, session_receiver) = mpsc::channel();
        let (client_sender, client_receiver) = mpsc::channel();
        let (server_sender, server_receiver) = mpsc::channel();
    }

    fn handle_from_server(&mut self, msg: Msg) -> Result<(), String> {

    }

    fn handle_from_client(&mut self, msg: Msg) -> Result<(), String> {

    }
}

impl Run for Session {
    fn run(self) {

    }
}

fn read_thread(source: Source, stream: TcpStream, session_sender: Sender<ProxyMsg>) {
    
}

fn write_thread() {

}
