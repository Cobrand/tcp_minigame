#![feature(proc_macro)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

#[cfg(feature = "sdl")]
extern crate sdl2;

mod color;
mod drawingboard;
pub mod game;
