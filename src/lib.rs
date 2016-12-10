#![feature(proc_macro)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

#[macro_use]
extern crate error_chain;

#[cfg(feature = "sdl")]
extern crate sdl2;

mod error;
mod color;
mod drawingboard;
pub mod game;
