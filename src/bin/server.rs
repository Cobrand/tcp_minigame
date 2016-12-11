extern crate tcp_minigame;
use tcp_minigame::game::*;

fn main() {
    start_server(61015, None).unwrap();
}
