use serde::{Serialize,Deserialize};
use std::fmt::Debug;

use super::color::Color;

#[cfg(feature = "sdl")]
use sdl2::render::Renderer;

use error::*;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DrawingBoard {
    pub data : Vec<Color<u8>>,
    pub width : u16,
    pub height : u16
}

impl DrawingBoard {
    pub fn new(width:u16,height:u16) -> DrawingBoard {
        let mut vec : Vec<Color<u8>> = vec![Color::<u8>::default(); width as usize * height as usize];
        DrawingBoard {
            data : vec,
            width: width,
            height: height
        }
    }

    fn pos_to_index(&self,pos:Position) -> Result<usize> {
        if pos.x >= self.width
        || pos.y >= self.height {
            Err(ErrorKind::OutOfBounds.into())
        } else {
            let index = pos.x as usize + self.width as usize * pos.y as usize;
            println!("pos {:?} ; index : {}",pos, index);
            Ok(index)
        }
    }

    fn index_to_pos(&self,index: usize) -> Result<Position> {
        let height : usize = index / self.width as usize;
        let width : usize= index - (height * self.width as usize);
        let height : u16= height as u16;
        let width : u16 = width as u16;
        if height >= self.height || width >= self.width {
            Err(ErrorKind::OutOfBounds.into())
        } else {
            Ok(Position::new(width,height))
        }
    }

    pub fn draw(&mut self, pos:Position, color:Color<u8>) -> Result<()> {
        let index = try!(self.pos_to_index(pos));

        self.data[index] = color;
        Ok(())
    }

    #[cfg(feature = "sdl")]
    pub fn renderer_draw(&self,renderer: &mut Renderer) {
        for (index, color) in self.data.iter().enumerate() {
            if let Ok(pos) = self.index_to_pos(index) {
                // if *color == Color::new(255, 255, 255) {
                //     println!("white {} {} {} !", self.width, self.height, index);
                // }
                renderer.set_draw_color(::sdl2::pixels::Color::RGB(color.r, color.g, color.b));
                renderer.fill_rect(::sdl2::rect::Rect::new(pos.x as i32 *16, pos.y as i32 * 16, 16, 16));
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct Position {
    pub x : u16,
    pub y : u16
}

impl Position {
    pub fn new(x:u16,y:u16) -> Position {
        Position {
            x:x,
            y:y,
        }
    }
}
