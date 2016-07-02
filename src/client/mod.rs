//! The structs representing a client's context.

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::net::{TcpStream, SocketAddr};
use std::thread;
use std::io::{Read, Write, Cursor};

use byteorder::{LittleEndian as LE, ReadBytesExt};

use msg::*;
use serial::*;
use crypto::Cipher;

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
        let (client_sender, client_receiver) = mpsc::channel::<Msg>();
        let (server_sender, server_receiver) = mpsc::channel::<Msg>();

        // Client read thread
        let c_r = client.try_clone().unwrap();
        let ss = session_sender.clone();
        let (client_read_cipher_sender, client_read_cipher_receiver) = mpsc::channel::<Option<Cipher>>();
        let (client_write_cipher_sender, client_write_cipher_receiver) = mpsc::channel::<Option<Cipher>>();
        let (server_read_cipher_sender, server_read_cipher_receiver) = mpsc::channel::<Option<Cipher>>();
        let (server_write_cipher_sender, server_write_cipher_receiver) = mpsc::channel::<Option<Cipher>>();

        thread::spawn(move|| {
            //read_thread(Side::Client, c_r, ss, )
        });

        thread::spawn(move|| {

        });

        unimplemented!()
    }

    fn handle_from_server(&mut self, msg: Msg) -> Result<(), String> {
        unimplemented!()
    }

    fn handle_from_client(&mut self, msg: Msg) -> Result<(), String> {
        unimplemented!()
    }
}

impl Run for Session {
    fn run(self) {

    }
}

enum Side {
    Server,
    Client
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
        match receiver.recv() {
            Ok(Some(c)) => cipher = Some(c),
            _ => {
                // An error ocurred
                error!("Unable to receive read cipher. Something else happened?");

                session_sender.send(ProxyMsg::ReadThreadClosed(side)).unwrap();
                return;
            }
        }
    } else {
        cipher = None;
    }

    loop {
        // Read the header.
        if let Err(e) = stream.read_exact(&mut header_buf[..4]) {
            error!("Unable to read header from read stream: {}", e);
            break;
        }

        if let Some(ref mut cipher) = cipher {
            // Decrypt
            cipher.codec(&mut header_buf[..]);
        }

        let mut header_cursor = Cursor::new(header_buf);
        let ty = header_cursor.read_u8().unwrap();
        let flags = header_cursor.read_u8().unwrap();
        let size = round_up(header_cursor.read_u16::<LE>().unwrap() - 4, 4);
        let mut read_buf: Vec<u8> = vec![0; size as usize];

        // Read the body.
        if let Err(e) = stream.read_exact(&mut read_buf[..(size as usize)]) {
            error!("Unable to read message body from read stream: {}", e);
            break;
        }

        if let Some(ref mut cipher) = cipher {
            cipher.codec(&mut header_buf[..]).unwrap();
        }

        let mut combined_buf = Vec::with_capacity(size as usize + 4);
        combined_buf.extend_from_slice(&header_buf[..]);
        combined_buf.extend_from_slice(&read_buf[..]);

        // Parse the total message.
        match Msg::deserialize(Cursor::new(combined_buf)) {
            Ok(m) => {
                match side {
                    Side::Server => {
                        // Special case: check if we got Welcome. If we did, send new cipher out
                        // and set our own cipher.
                        match (&m, cipher.is_none()) {
                            (&Msg::Welcome(_, ref w), true) => {
                                cipher = Some(Cipher::new(w.server_seed));
                                if let Some(ref sender) = write_cipher_sender {
                                    sender.send(Some(Cipher::new(w.client_seed))).unwrap();
                                }
                            },
                            _ => ()
                        }

                        session_sender.send(ProxyMsg::Server(m)).unwrap();
                    },
                    Side::Client => session_sender.send(ProxyMsg::Client(m)).unwrap()
                }
            },
            Err(e) => {
                error!("Unable to deserialize full message from read stream: {}", e);
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
                write_cipher_receiver: Option<Receiver<Option<Cipher>>>,
                read_cipher_sender: Option<Sender<Option<Cipher>>>) {
    let mut cipher: Option<Cipher>;

    if let Some(ref receiver) = write_cipher_receiver {
        match receiver.recv() {
            Ok(Some(c)) => cipher = Some(c),
            _ => {
                error!("Unable to receive write cipher on write thread. Something happened elsewhere?");
                session_sender.send(ProxyMsg::WriteThreadClosed(side)).unwrap();
                return;
            }
        }
    } else {
        cipher = None;
    }

    loop {
        match msg_recv.recv() {
            Ok(m) => {
                let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
                m.serialize(&mut cursor).unwrap();
                let mut buf: Vec<u8> = cursor.into_inner();

                if let Some(ref mut cipher) = cipher {
                    cipher.codec(&mut buf[..]);
                }
                stream.write_all(&buf[..]);

                match (&m, cipher.is_none()) {
                    (&Msg::Welcome(_, ref w), true) => {
                        cipher = Some(Cipher::new(w.server_seed));
                        if let Some(ref sender) = read_cipher_sender {
                            sender.send(Some(Cipher::new(w.client_seed))).unwrap();
                        }
                    },
                    _ => ()
                }
            },
            Err(e) => {
                error!("Error receiving message to write in write thread: {}", e);
                break;
            }
        }
    }

    session_sender.send(ProxyMsg::WriteThreadClosed(side)).unwrap();
}
