extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color as SdlColor;

use bincode::serde::{deserialize_from,serialize_into,SerializeError,DeserializeError};
use bincode::SizeLimit;

use std::result::Result as StdResult;
use std::io::{Error as IoError,ErrorKind as IoErrorKind};
use std::net::{ToSocketAddrs,TcpStream,TcpListener};

use super::drawingboard::*;
use super::color::Color;
use super::messages::ClientMessage;

use error::*;

pub fn start_server(port: u16, initial_data: Option<DrawingBoard>) -> Result<()> {
    let mut tcp_streams : Vec<TcpStream> = vec![];
    let mut to_remove : Vec<usize> = vec!();
    let mut drawing_board = initial_data.unwrap_or(DrawingBoard {
        data: vec![Color::new(0,0,0); 1200],
        width: 40,
        height: 30,
    });
    let listener : TcpListener = try!(TcpListener::bind(("0.0.0.0",port)));
    listener.set_nonblocking(true);
    loop {
        let mut update = false;
        while let Ok((mut new_stream,_addr)) = listener.accept() {
            if let Ok(_) = serialize_into(&mut new_stream, &drawing_board, SizeLimit::Infinite) {;
                println!("Accepted connection from {}", new_stream.peer_addr().unwrap());
                new_stream.set_nonblocking(false);
                new_stream.set_read_timeout(Some(::std::time::Duration::from_millis(2))).unwrap();
                tcp_streams.push(new_stream);
            } else {
                println!("An error occured when trying to accept connection from {}", new_stream.peer_addr().unwrap());
            }
        }
        //println!("connected: {}",tcp_streams.len());
        for (index, mut stream) in tcp_streams.iter_mut().enumerate() {
            let read_result : StdResult<ClientMessage<Color<u8>>,_> =
                deserialize_from(&mut stream, SizeLimit::Bounded(1024));
            match read_result {
                Ok(client_message) => {
                    println!("message !");
                    match drawing_board.draw(client_message.position, client_message.color) {
                        Ok(()) => {
                            update = true;
                        },
                        Err(_) => {
                            /* ignore OutOfBounds error for now */
                        }
                    };
                    // println!("{:?}",drawing_board);
                },
                Err(DeserializeError::IoError(io_error)) => {
                    match io_error.kind() {
                        IoErrorKind::ConnectionAborted |
                        IoErrorKind::BrokenPipe |
                        IoErrorKind::Interrupted |
                        IoErrorKind::UnexpectedEof => {
                            println!("Some client disconnected");
                            to_remove.push(index);
                        },
                        _ => {
                            //println!("IoError: {:?}",e);
                        },
                    };
                },
                Err(e) => {
                    /* ignore other errors */
                }
            }
        };
        for &index in &to_remove {
            tcp_streams.swap_remove(index);
        };
        to_remove.clear();
        if update {
            for mut stream in &mut tcp_streams {
                let write_result : StdResult<(),_> =
                    serialize_into(&mut stream, &drawing_board, SizeLimit::Infinite);
                match write_result {
                    Ok(_) => {
                        println!("send updated world !");
                        /* Everything went well! */
                    },
                    Err(_) => {
                        /* an error happened :( We'll ignore it for now */
                    }
                }
            }
        }
    }
}

#[cfg(feature = "sdl")]
pub fn start_client<A: ToSocketAddrs>(target_ip: A) -> Result<()> {
    let mut tcp_stream = try!(TcpStream::connect(target_ip));
    println!("Connected to remote server");
    // let message : ClientMessage<Color<u8>> = ClientMessage {
    //     color: Color::new(0,128,255),
    //     position: Position::new(5,9),
    // };
    let mut drawing_board : DrawingBoard =
        deserialize_from(&mut tcp_stream, SizeLimit::Bounded(10240)).unwrap();
    tcp_stream.set_nonblocking(true);
    //let write_result : StdResult<(),_> = serialize_into(&mut tcp_stream, &message, SizeLimit::Infinite);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("tcp_minigame",
                                        drawing_board.width as u32 * 16,
                                        drawing_board.height as u32 * 16)
        .position_centered()
        .build()
        .unwrap();
    let mut renderer = window.renderer().present_vsync().build().unwrap();

    renderer.set_draw_color(SdlColor::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            use sdl2::mouse::MouseButton;
            use std::cmp::max;
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left , x: x , y: y,  ..} => {
                    let message : ClientMessage<Color<u8>> = ClientMessage {
                        color: Color::new(255,255,255),
                        position: Position::new(max(0,x) as u16 / 16, max(0,y) as u16 / 16),
                    };
                    println!("message: {:?}",message);
                    let write_result : StdResult<(),_> =
                        serialize_into(&mut tcp_stream, &message, SizeLimit::Infinite);
                    println!("write_result : {:?}",write_result);
                },
                _ => {}
            }
        }
        let read_result : StdResult<DrawingBoard,_> =
            deserialize_from(&mut tcp_stream, SizeLimit::Bounded(10240));
        if let Ok(new_drawing_board) = read_result {
            println!("received board !");
            drawing_board = new_drawing_board;
            //println!("{:?}: {}x{}",drawing_board.data.len(),drawing_board.width,drawing_board.height);
        };
        renderer.clear();
        drawing_board.renderer_draw(&mut renderer);
        renderer.present();
        // The rest of the game loop goes here...
    }
    Ok(())
}

#[cfg(not(feature = "sdl"))]
pub fn start_client<A: ToSocketAddrs>(target_ip: A) {
    println!("Cannot start graphic client without sdl2 linked");
    ::std::process::exit(1);
}
