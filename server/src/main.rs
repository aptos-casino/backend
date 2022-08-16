use std::sync::{Arc, Mutex};

mod aptos_rpc;
mod event_parser;
mod loops;
mod utils;

type GameVecMutex = Arc<Mutex<Vec<event_parser::Game>>>;

#[tokio::main]
async fn main() {
   
    let mut games: GameVecMutex = Arc::new(Mutex::new(vec![]));
    
    let res = tokio::try_join!(loops::event_parsing_loop(
        &mut games
    ));
    println!("Error occurs! {}", res.err().unwrap());
}
