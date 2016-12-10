use serde::{Serialize,Deserialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DrawingBoard<C : Default> {
    pub data : Vec<C>,
    pub width : u16,
    pub height : u16
}

impl<C : Default+Clone+Copy> DrawingBoard<C> {
    pub fn new(width:u16,height:u16) -> DrawingBoard<C> {
        let mut vec : Vec<C> = vec![C::default(); width as usize * height as usize];
        DrawingBoard {
            data : vec,
            width: width,
            height: height
        }
    }

    fn pos_to_index(&self,pos:Position) -> Result<usize,&'static str> {
        if pos.x >= self.width
        || pos.y >= self.height {
            Err("out of range")
        } else {
            Ok(pos.x as usize + self.width as usize * pos.y as usize)
        }
    }

    pub fn draw(&mut self,pos:Position,color:C) -> Result<(),&'static str> {
        let index = try!(self.pos_to_index(pos));
        self.data[index] = color;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct Position {
    pub x : u16,
    pub y : u16
}
