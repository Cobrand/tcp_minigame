use bincode::serde::{deserialize_from,serialize_into,SerializeError,DeserializeError};
use bincode::SizeLimit;

use std::result::Result as StdResult;
use std::io::{Error as IoError,ErrorKind as IoErrorKind};
use std::net::{ToSocketAddrs,TcpStream,TcpListener};

use super::drawingboard::DrawingBoard;
use super::color::Color;
use super::messages::ClientMessage;

use error::*;

pub fn start_server(port: u16, initial_data: Option<DrawingBoard<Color<u8>>>) -> Result<()> {
    let mut tcp_streams : Vec<TcpStream> = vec![];
    let mut to_remove : Vec<usize> = vec!();
    let listener : TcpListener = try!(TcpListener::bind(("0.0.0.0",port)));
    listener.set_nonblocking(true);
    loop {
        //println!("loop");
        while let Ok((new_stream,_addr)) = listener.accept() {
            println!("Accepted connection from {}", new_stream.peer_addr().unwrap());
            let _ = new_stream.set_nodelay(true);
            tcp_streams.push(new_stream);
        }
        for (index, mut stream) in tcp_streams.iter_mut().enumerate() {
            //println!("m? {:?}",stream.peer_addr());
            let read_result : StdResult<ClientMessage<Color<u8>>,_> =
                deserialize_from(&mut stream, SizeLimit::Bounded(1024));
            match read_result {
                Ok(client_message) => {
                    println!("Received message from {:?}: {:?}",
                             stream.peer_addr(),
                             client_message);
                },
                Err(DeserializeError::IoError(io_error)) => {
                    match io_error.kind() {
                        IoErrorKind::ConnectionAborted |
                        IoErrorKind::BrokenPipe |
                        IoErrorKind::Interrupted |
                        IoErrorKind::UnexpectedEof => {
                            println!("Client {} disconnected",stream.peer_addr().unwrap());
                            to_remove.push(index);
                        },
                        _ => {},
                    };
                },
                Err(_) => {
                    /* ignore other errors */
                }
            }
        };
        for &index in &to_remove {
            tcp_streams.swap_remove(index);
        };
        to_remove.clear();
    }
}

#[cfg(feature = "sdl")]
pub fn start_client<A: ToSocketAddrs>(target_ip: A) -> Result<()> {
    let mut tcp_stream = try!(TcpStream::connect(target_ip));
    tcp_stream.set_nodelay(true);
    println!("Connected to remote server");
    let message : ClientMessage<Color<u8>> = ClientMessage {
        color: Color::new(0,128,255),
        position: (5, 9),
    };
    let write_result : StdResult<(),_> = serialize_into(&mut tcp_stream, &message, SizeLimit::Infinite);
    Ok(())
}

#[cfg(not(feature = "sdl"))]
pub fn start_client<A: ToSocketAddrs>(target_ip: A) {
    println!("Cannot start graphic client without sdl2 linked");
    ::std::process::exit(1);
}
