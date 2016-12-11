use serde::{Serialize,Deserialize};

use error::*;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMessage<C : Serialize+Deserialize+Debug> {
    pub color: C,
    pub position: (u16,u16),
}

