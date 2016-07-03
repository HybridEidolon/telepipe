//! The structs representing a client's context.

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::net::{TcpStream, SocketAddr, Shutdown};
use std::thread;
use std::io::{Read, Write, Cursor};
use std::error::Error;

use byteorder::{LittleEndian as LE, ReadBytesExt};

use msg::*;
use msg::common::*;
use serial::*;
use crypto::Cipher;

pub trait Run {
    fn run(self);
}

pub struct Session {
    session_receiver: Receiver<ProxyMsg>,
    client_stream: TcpStream,
    server_stream: TcpStream,
    client_write_sender: Sender<Msg>,
    server_write_sender: Sender<Msg>
}

pub struct Client {

}

pub struct Server {

}

enum ProxyMsg {
    Server(Msg),
    Client(Msg),
    ReadThreadClosed(Side),
    WriteThreadClosed(Side)
}

impl Session {
    pub fn new(client: TcpStream, initial_server_addr: SocketAddr) -> Result<Session, String> {
        // Start new threads to process reads and writes, and use channels to handle incoming messages.
        // Use this session to handle messages from both the client and from the server.

        // Connect to the server.
        let server = TcpStream::connect(initial_server_addr).unwrap();

        let (session_sender, session_receiver) = mpsc::channel::<ProxyMsg>();

        let client_write_sender = create_connection_threads(Side::Client, client.try_clone().unwrap(), session_sender.clone());
        let server_write_sender = create_connection_threads(Side::Server, server.try_clone().unwrap(), session_sender.clone());

        Ok(Session {
            server_stream: server,
            client_stream: client,
            session_receiver: session_receiver,
            client_write_sender: client_write_sender,
            server_write_sender: server_write_sender
        })
    }

    fn handle_from_server(&mut self, msg: Msg) {
        info!("Server: {:?}", msg);
        match msg {
            Msg::LoginWelcome(f, w) => {
                // Craft our own LoginWelcome and send it to the client instead.
                let my_w = Msg::LoginWelcome(0, Welcome {
                    copyright: LOGIN_WELCOME_COPYRIGHT.to_string(),
                    ..Default::default()
                });
                self.client_write_sender.send(my_w).unwrap();
            },
            Msg::Redirect4(r) => {
                unimplemented!()
            },
            Msg::Redirect6(r) => {
                unimplemented!()
            },
            m => self.client_write_sender.send(m).unwrap()
        }
    }

    fn handle_from_client(&mut self, msg: Msg) {
        info!("Client: {:?}", msg);
        // Eventually we'll do more here, but this is the bare minimum.
        self.server_write_sender.send(msg).unwrap();
    }
}

impl Run for Session {
    fn run(mut self) {
        loop {
            // Read from session messages
            match self.session_receiver.recv() {
                Ok(m) => match m {
                    ProxyMsg::Server(m) => self.handle_from_server(m),
                    ProxyMsg::Client(m) => self.handle_from_client(m),
                    ProxyMsg::ReadThreadClosed(s) if s == Side::Server => {
                        info!("Server read thread closed");
                        self.client_stream.shutdown(Shutdown::Both).unwrap();
                        self.server_stream.shutdown(Shutdown::Both).unwrap();
                        break;
                    },
                    ProxyMsg::ReadThreadClosed(s) if s == Side::Client => {
                        info!("Client read thread closed");
                        self.client_stream.shutdown(Shutdown::Both).unwrap();
                        self.server_stream.shutdown(Shutdown::Both).unwrap();
                        break;
                    },
                    ProxyMsg::WriteThreadClosed(s) if s == Side::Server => {
                        info!("Server write thread closed");
                        self.client_stream.shutdown(Shutdown::Both).unwrap();
                        self.server_stream.shutdown(Shutdown::Both).unwrap();
                        break;
                    },
                    ProxyMsg::WriteThreadClosed(s) if s == Side::Client => {
                        info!("Server write thread closed");
                        self.client_stream.shutdown(Shutdown::Both).unwrap();
                        self.server_stream.shutdown(Shutdown::Both).unwrap();
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

    if let Some(receiver) = read_cipher_receiver {
        // Block to receive the read cipher.
        debug!("{:?} Read thread on {} is waiting for read cipher", side, stream.peer_addr().unwrap());
        match receiver.recv() {
            Ok(Some(c)) => cipher = Some(c),
            _ => {
                // An error ocurred
                error!("Unable to receive read cipher. Something else happened?");

                session_sender.send(ProxyMsg::ReadThreadClosed(side)).unwrap();
                return;
            }
        }
        debug!("Got {:?} read thread cipher on {}", side, stream.peer_addr().unwrap());
    } else {
        debug!("Read thread on {} does not need a read cipher yet (server read)", stream.peer_addr().unwrap());
        cipher = None;
    }

    loop {
        // Read the header.
        debug!("Reading header");
        if let Err(e) = stream.read_exact(&mut header_buf[..4]) {
            error!("Unable to read header from {:?} read stream: {}", side, e);
            break;
        }

        debug!("Pre-decrypted header buffer: {:?}", &header_buf[..]);

        if let Some(ref mut cipher) = cipher {
            // Decrypt
            cipher.codec(&mut header_buf[..]);
        } else {
            debug!("{:?} Read cipher was not available, so no codec needed on header", side);
        }

        debug!("Post-decrypted header buffer: {:?}", &header_buf[..]);

        let mut header_cursor = Cursor::new(header_buf);
        let ty = header_cursor.read_u8().unwrap();
        let flags = header_cursor.read_u8().unwrap();
        let size_as_written = header_cursor.read_u16::<LE>().unwrap();
        let size = round_up(size_as_written - 4, 4);
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
                    _ => ()
                }

                let result = match side {
                    Side::Server => session_sender.send(ProxyMsg::Server(m)),
                    Side::Client => session_sender.send(ProxyMsg::Client(m))
                };
                match result {
                    Err(_) => {
                        error!("Unable to send message to session sender.");
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

    session_sender.send(ProxyMsg::ReadThreadClosed(side)).unwrap();
}

fn write_thread(side: Side,
                mut stream: TcpStream,
                msg_recv: Receiver<Msg>,
                session_sender: Sender<ProxyMsg>,
                read_cipher_sender: Option<Sender<Option<Cipher>>>,
                write_cipher_receiver: Option<Receiver<Option<Cipher>>>) {
    let mut cipher: Option<Cipher>;

    if let Some(ref receiver) = write_cipher_receiver {
        debug!("{:?} Write thread on {} is waiting for write cipher", side, stream.peer_addr().unwrap());
        match receiver.recv() {
            Ok(Some(c)) => cipher = Some(c),
            _ => {
                error!("Unable to receive write cipher on write thread. Something happened elsewhere?");
                session_sender.send(ProxyMsg::WriteThreadClosed(side)).unwrap();
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
                    cipher.codec(&mut buf[..]);
                } else {
                    debug!("Writing unencrypted message since no cipher available yet.");
                }
                debug!("Message buffer size: {}", buf.len());
                stream.write_all(&buf[..]);

                match (&m, cipher.is_none()) {
                    (&Msg::LoginWelcome(_, ref w), true) => {
                        cipher = Some(Cipher::new(w.server_seed));
                        if let Some(ref sender) = read_cipher_sender {
                            debug!("LoginWelcome message is being written, sending cipher to reader thread.");
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

    session_sender.send(ProxyMsg::WriteThreadClosed(side)).unwrap();
}