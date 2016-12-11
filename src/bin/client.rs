extern crate tcp_minigame;
use tcp_minigame::game::*;

fn main(){
    start_client("127.0.0.1:61015").unwrap();
}
