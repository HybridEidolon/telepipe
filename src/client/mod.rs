//! The structs representing a client's context.

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::net::{TcpStream, SocketAddr, Shutdown, SocketAddrV4};
use std::thread;
use std::io::{Read, Write, Cursor};
use std::str::FromStr;

use byteorder::{LittleEndian as LE, ReadBytesExt};
use rand::random;

use msg::*;
use msg::common::*;
use serial::*;
use crypto::Cipher;
use session::spawn_new_session;

pub trait Run {
    fn run(self);
}

pub struct Session {
    session_receiver: Receiver<ProxyMsg>,
    client_stream: TcpStream,
    server_stream: TcpStream,
    client_write_sender: Sender<Msg>,
    server_write_sender: Sender<Msg>,
    server_addr: SocketAddr,
    welcome_sent: bool,
    ship_welcome_sent: bool,
    disconnected: bool,
    local_server_addr: String,
    local_bind_addr: String
}

enum ProxyMsg {
    Server(Msg),
    Client(Msg),
    ReadThreadClosed(Side, SocketAddr),
    WriteThreadClosed(Side, SocketAddr)
}

impl Session {
    pub fn new(client: TcpStream, initial_server_addr: SocketAddr, local_server_addr: String, local_bind_addr: String) -> Result<Session, String> {
        // Start new threads to process reads and writes, and use channels to handle incoming messages.
        // Use this session to handle messages from both the client and from the server.

        // Connect to the server.
        let server: TcpStream = TcpStream::connect(initial_server_addr).unwrap();

        let (session_sender, session_receiver) = mpsc::channel::<ProxyMsg>();

        let client_write_sender = create_connection_threads(Side::Client, client.try_clone().unwrap(), session_sender.clone());
        let server_write_sender = create_connection_threads(Side::Server, server.try_clone().unwrap(), session_sender.clone());

        Ok(Session {
            server_stream: server,
            client_stream: client,
            session_receiver: session_receiver,
            client_write_sender: client_write_sender,
            server_write_sender: server_write_sender,
            server_addr: initial_server_addr,
            welcome_sent: false,
            ship_welcome_sent: false,
            disconnected: false,
            local_server_addr: local_server_addr,
            local_bind_addr: local_bind_addr
        })
    }

    fn handle_from_server(&mut self, msg: Msg) {
        info!("Server: {:?}", msg);
        match msg {
            Msg::LoginWelcome(_, _) => {
                info!("Welcome to a login server!");
                if !self.welcome_sent {
                    // Craft our own LoginWelcome and send it to the client instead.
                    let my_w = Msg::LoginWelcome(0, Welcome {
                        copyright: LOGIN_WELCOME_COPYRIGHT.to_string(),
                        client_seed: random::<u32>(),
                        server_seed: random::<u32>()
                    });
                    self.client_write_sender.send(my_w).unwrap();
                    self.welcome_sent = true;
                }
            },
            Msg::ShipWelcome(_, _) => {
                info!("Welcome to a lobby server!");
                if !self.ship_welcome_sent {
                    let my_w = Msg::ShipWelcome(0, Welcome {
                        copyright: SHIP_WELCOME_COPYRIGHT.to_string(),
                        client_seed: random::<u32>(),
                        server_seed: random::<u32>()
                    });
                    self.client_write_sender.send(my_w).unwrap();
                    self.welcome_sent = true;
                }
            },
            Msg::Redirect4(r) => {
                let new_port = spawn_new_session(format!("{}:0", self.local_bind_addr).as_str(), r.socket_addr, self.local_server_addr.clone(), self.local_bind_addr.clone(), true).unwrap();
                let rr = Msg::Redirect4(Redirect4 { socket_addr: SocketAddrV4::from_str(&format!("{}:{}", self.local_server_addr, new_port)).unwrap() });
                self.client_write_sender.send(rr).unwrap();
            },
            Msg::Redirect6(r) => {
                let new_port = spawn_new_session(format!("{}:0", self.local_bind_addr).as_str(), r.socket_addr, self.local_server_addr.clone(), self.local_bind_addr.clone(), true).unwrap();
                let rr = Msg::Redirect4(Redirect4 { socket_addr: SocketAddrV4::from_str(&format!("{}:{}", self.local_server_addr, new_port)).unwrap() });
                self.client_write_sender.send(rr).unwrap();
            },
            m => self.client_write_sender.send(m).unwrap()
        }
    }

    fn handle_from_client(&mut self, msg: Msg) {
        info!("Client: {:?}", msg);
        match msg {
            m => self.server_write_sender.send(m).unwrap()
        }
    }
}

impl Run for Session {
    fn run(mut self) {
        loop {
            // Read from session messages
            match self.session_receiver.recv() {
                Ok(m) => match m {
                    ProxyMsg::Server(m) => {
                        self.handle_from_server(m);
                        if self.disconnected { break }
                    },
                    ProxyMsg::Client(m) => {
                        self.handle_from_client(m);
                        if self.disconnected { break }
                    },
                    ProxyMsg::ReadThreadClosed(s, peer_addr) if s == Side::Server => {
                        // check if the peer address is the current peer address
                        if peer_addr == self.server_addr {
                            info!("Server read thread closed");
                            break;
                        } else {
                            info!("Old server read thread closed");
                        }
                    },
                    ProxyMsg::ReadThreadClosed(s, _peer_addr) if s == Side::Client => {
                        // The client should never disconnect because of redirect.
                        info!("Client read thread closed");
                        break;
                    },
                    ProxyMsg::WriteThreadClosed(s, peer_addr) if s == Side::Server => {
                        // check if the peer address is the current peer address
                        if peer_addr == self.server_addr {
                            info!("Server write thread closed");
                            break;
                        } else {
                            info!("Old server write thread closed");
                        }
                    },
                    ProxyMsg::WriteThreadClosed(s, _peer_addr) if s == Side::Client => {
                        // The client should never disconnect because of redirect.
                        info!("Client write thread closed");
                        break;
                    },
                    _ => ()
                },
                Err(_) => {
                    info!("All senders for this session have been dropped. Closing session.");
                    break;
                }
            }
        }

        info!("Exiting session.");
        self.client_stream.shutdown(Shutdown::Both).unwrap();
        self.server_stream.shutdown(Shutdown::Both).unwrap();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Side {
    Server,
    Client
}

fn create_connection_threads(side: Side, client: TcpStream, session_sender: Sender<ProxyMsg>) -> Sender<Msg> {
    let (cipher_sender, cipher_receiver) = mpsc::channel();

    let (write_sender, write_receiver) = mpsc::channel();

    if side == Side::Server {
        let c_r = client.try_clone().unwrap();
        let ss = session_sender.clone();
        thread::spawn(move|| { read_thread(side, c_r, ss, Some(cipher_sender), None); });
        let c_r = client.try_clone().unwrap();
        let ss = session_sender.clone();
        thread::spawn(move|| { write_thread(side, c_r, write_receiver, ss, None, Some(cipher_receiver)); });
    } else {
        let c_r = client.try_clone().unwrap();
        let ss = session_sender.clone();
        thread::spawn(move|| { read_thread(side, c_r, ss, None, Some(cipher_receiver)); });
        let c_r = client.try_clone().unwrap();
        let ss = session_sender.clone();
        thread::spawn(move|| { write_thread(side, c_r, write_receiver, ss, Some(cipher_sender), None); });
    }

    write_sender
}

fn read_thread(side: Side,
               mut stream: TcpStream,
               session_sender: Sender<ProxyMsg>,
               write_cipher_sender: Option<Sender<Option<Cipher>>>,
               read_cipher_receiver: Option<Receiver<Option<Cipher>>>) {
    let mut cipher: Option<Cipher>;
    let mut header_buf: [u8; 4] = [0; 4];
    let peer_addr = match stream.peer_addr() {
        Ok(a) => a,
        Err(_) => {
            // okay so... idk what to do
            session_sender.send(ProxyMsg::ReadThreadClosed(side, stream.local_addr().unwrap())).unwrap();
            return
        }
    };

    info!("Read thread on {} open.", peer_addr);

    if let Some(receiver) = read_cipher_receiver {
        // Block to receive the read cipher.
        debug!("{:?} Read thread on {} is waiting for read cipher", side, peer_addr);
        match receiver.recv() {
            Ok(Some(c)) => cipher = Some(c),
            _ => {
                // An error ocurred
                error!("Unable to receive read cipher. Something else happened?");

                session_sender.send(ProxyMsg::ReadThreadClosed(side, peer_addr)).unwrap();
                return;
            }
        }
        debug!("Got {:?} read thread cipher on {}", side, peer_addr);
    } else {
        debug!("Read thread on {} does not need a read cipher yet (server read)", peer_addr);
        cipher = None;
    }

    loop {
        // Read the header.
        debug!("{:?} side Reading header", side);
        if let Err(e) = stream.read_exact(&mut header_buf[..4]) {
            error!("Unable to read header from {:?} read stream at {}: {}", side, peer_addr, e);
            break;
        }

        debug!("Pre-decrypted header buffer: {:?}", &header_buf[..]);

        if let Some(ref mut cipher) = cipher {
            // Decrypt
            cipher.codec(&mut header_buf[..]).unwrap();
        } else {
            debug!("{:?} Read cipher was not available, so no codec needed on header", side);
        }

        debug!("Post-decrypted header buffer: {:?}", &header_buf[..]);

        let mut header_cursor = Cursor::new(header_buf);
        let ty = header_cursor.read_u8().unwrap();
        let flags = header_cursor.read_u8().unwrap();
        let size_as_written = header_cursor.read_u16::<LE>().unwrap();
        let size = if size_as_written <= 4 { 0 } else { round_up(size_as_written - 4, 4) };
        let mut read_buf: Vec<u8> = vec![0; size as usize];

        debug!("Header: ty={} flags={} size_as_written={} size={}", ty, flags, size_as_written, size);
        // Read the body.
        if let Err(e) = stream.read_exact(&mut read_buf[..(size as usize)]) {
            error!("Unable to read message body from {:?} read stream: {}", side, e);
            break;
        }

        debug!("Body read");

        if let Some(ref mut cipher) = cipher {
            cipher.codec(&mut read_buf[..]).unwrap();
        } else {
            debug!("{:?} Read cipher was not available, so no codec needed on body", side);
        }

        let mut combined_buf = Vec::with_capacity(size as usize + 4);
        combined_buf.extend_from_slice(&header_buf[..]);
        combined_buf.extend_from_slice(&read_buf[..]);

        // Parse the total message.
        match Msg::deserialize(Cursor::new(combined_buf)) {
            Ok(m) => {
                // Special case: check if we got LoginWelcome. If we did, send new cipher out
                // and set our own cipher.
                match (&m, cipher.is_none()) {
                    (&Msg::LoginWelcome(_, ref w), true) => {
                        cipher = Some(Cipher::new(w.server_seed));
                        if let Some(ref sender) = write_cipher_sender {
                            debug!("LoginWelcome message received, sending cipher to writer thread.");
                            sender.send(Some(Cipher::new(w.client_seed))).unwrap();
                        }
                    },
                    (&Msg::ShipWelcome(_, ref w), true) => {
                        cipher = Some(Cipher::new(w.server_seed));
                        if let Some(ref sender) = write_cipher_sender {
                            debug!("ShipWelcome message received, sending cipher to writer thread.");
                            sender.send(Some(Cipher::new(w.client_seed))).unwrap();
                        }
                    },
                    _ => ()
                }

                let result = match side {
                    Side::Server => session_sender.send(ProxyMsg::Server(m)),
                    Side::Client => session_sender.send(ProxyMsg::Client(m))
                };
                match result {
                    Err(_) => {
                        // Session is closed.
                        //error!("Unable to send message to session sender.");
                        break;
                    },
                    _ => ()
                }
            },
            Err(e) => {
                error!("Unable to deserialize full message from {:?} read stream: {}", side, e);
                break;
            }
        }

        // Start over...
    }

    match session_sender.send(ProxyMsg::ReadThreadClosed(side, peer_addr)) {
        _ => ()
    }
}

fn write_thread(side: Side,
                mut stream: TcpStream,
                msg_recv: Receiver<Msg>,
                session_sender: Sender<ProxyMsg>,
                read_cipher_sender: Option<Sender<Option<Cipher>>>,
                write_cipher_receiver: Option<Receiver<Option<Cipher>>>) {
    let mut cipher: Option<Cipher>;
    let peer_addr = match stream.peer_addr() {
        Ok(a) => a,
        Err(_) => {
            // okay so... idk what to do
            session_sender.send(ProxyMsg::WriteThreadClosed(side, stream.local_addr().unwrap())).unwrap();
            return
        }
    };

    info!("Write thread on {} open.", peer_addr);

    if let Some(ref receiver) = write_cipher_receiver {
        debug!("{:?} Write thread on {} is waiting for write cipher", side, stream.peer_addr().unwrap());
        match receiver.recv() {
            Ok(Some(c)) => cipher = Some(c),
            _ => {
                error!("Unable to receive write cipher on write thread. Something happened elsewhere?");
                session_sender.send(ProxyMsg::WriteThreadClosed(side, peer_addr)).unwrap();
                return;
            }
        }
        debug!("Got {:?} write thread cipher on {}", side, stream.peer_addr().unwrap());
    } else {
        debug!("{:?} Write thread on {} does not need a write cipher yet (client write)", side, stream.peer_addr().unwrap());
        cipher = None;
    }

    loop {
        match msg_recv.recv() {
            Ok(m) => {
                debug!("{:?} Writing a message {:?}", side, m);
                let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
                m.serialize(&mut cursor).unwrap();
                let mut buf: Vec<u8> = cursor.into_inner();

                if let Some(ref mut cipher) = cipher {
                    cipher.codec(&mut buf[..]).unwrap();
                } else {
                    debug!("Writing unencrypted message since no cipher available yet.");
                }
                debug!("Message buffer size: {}", buf.len());
                match stream.write_all(&buf[..]) {
                    Ok(_) => (),
                    Err(e) => {
                        error!("Unable to write full message to {:?} stream at {}: {}", side, peer_addr, e);
                        break
                    }
                }

                match (&m, cipher.is_none()) {
                    (&Msg::LoginWelcome(_, ref w), true) => {
                        cipher = Some(Cipher::new(w.server_seed));
                        if let Some(ref sender) = read_cipher_sender {
                            debug!("LoginWelcome message is being written, sending cipher to reader thread.");
                            sender.send(Some(Cipher::new(w.client_seed))).unwrap();
                        }
                    },
                    (&Msg::ShipWelcome(_, ref w), true) => {
                        cipher = Some(Cipher::new(w.server_seed));
                        if let Some(ref sender) = read_cipher_sender {
                            debug!("ShipWelcome message is being written, sending cipher to reader thread.");
                            sender.send(Some(Cipher::new(w.client_seed))).unwrap();
                        }
                    },
                    _ => ()
                }
            },
            Err(e) => {
                error!("Error receiving message to write in {:?} write thread: {}", side, e);
                break;
            }
        }
    }

    match session_sender.send(ProxyMsg::WriteThreadClosed(side, peer_addr)) {
        _ => ()
    }
}
