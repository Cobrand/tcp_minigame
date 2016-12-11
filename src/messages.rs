use serde::{Serialize,Deserialize};
use super::drawingboard::Position;

use error::*;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMessage<C : Serialize+Deserialize+Debug> {
    pub color: C,
    pub position: Position,
}

