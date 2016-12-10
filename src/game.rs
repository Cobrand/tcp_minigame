use std::net::ToSocketAddrs;
use super::drawingboard::DrawingBoard;
use super::color::Color;

pub fn start_server(port:u16, initial_data: Option<DrawingBoard<Color<u8>>>){

}

#[cfg(feature = "sdl")]
pub fn start_client<A: ToSocketAddrs>(target_ip: A, target_port:u16) {
    
}

#[cfg(not(feature = "sdl"))]
pub fn start_client<A: ToSocketAddrs>(target_ip: A, target_port:u16) {
    println!("Cannot start graphic client without sdl2 linked");
    ::std::process::exit(1);
}
