use serde::{Serialize,Deserialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Debug)]
pub struct Color<T> where T:Serialize+Deserialize+PartialEq+Debug+Clone+Copy {
    r:T,
    g:T,
    b:T,
}

impl Default for Color<u8> {
    fn default() -> Color<u8> {
        Color {
            r:0xFF,
            g:0xFF,
            b:0xFF,
        }
    }
}
