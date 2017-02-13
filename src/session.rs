//! Spawns a thread that wraps a TCP Listener socket with a new destination.

use std::io;
use std::fmt::Debug;
use std::net::{TcpListener, ToSocketAddrs, SocketAddr};
use std::thread;

use client::{Session, Run};

/// Spawn a session listener thread on the addresses given. If the local port is 0, a port will be
/// picked. The port of the local listener is returned.
pub fn spawn_new_session<A: ToSocketAddrs, B: ToSocketAddrs + Debug>(local_addr: A,
                                                                     server_addr: B,
                                                                     local_server_addr: String,
                                                                     local_bind_addr: String,
                                                                     kill_on_connect: bool)
                                                                     -> io::Result<u16> {
    let tcp_listener: TcpListener = TcpListener::bind(local_addr)?;
    let port = tcp_listener.local_addr()?.port();
    let server_socket_addr = server_addr.to_socket_addrs()?
        .next()
        .ok_or(io::Error::new(io::ErrorKind::AddrNotAvailable,
                              format!("Address {:?} invalid", server_addr)))?;

    thread::spawn(move || {
        session_thread(tcp_listener,
                       server_socket_addr,
                       local_server_addr,
                       local_bind_addr,
                       kill_on_connect);
    });

    Ok(port)
}

fn session_thread(listener: TcpListener,
                  server_addr: SocketAddr,
                  local_server_addr: String,
                  local_bind_addr: String,
                  kill_on_connect: bool) {
    let port = listener.local_addr().unwrap().port();
    info!("TCP listener thread live on port {:?}, kill_on_connect={}",
          port,
          kill_on_connect);

    for s in listener.incoming() {
        match s {
            Ok(s) => {
                info!("Connected {:?} on port {:?}.", s.peer_addr(), port);
                let session: Session = Session::new(s,
                                                    server_addr,
                                                    local_server_addr.clone(),
                                                    local_bind_addr.clone())
                    .unwrap();
                thread::spawn(move || {
                    info!("Session spawned.");
                    session.run();
                });
                if kill_on_connect {
                    break;
                }
            }
            Err(_) => {
                error!("Error in TCP listener");
                break;
            }
        }
    }
}
