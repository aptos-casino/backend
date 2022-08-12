use std::sync::{Arc, Mutex};

mod aptos_rpc;
mod utils;
mod event_parser;
mod loops;

type GameVecMutex = Arc<Mutex<Vec<event_parser::Game>>>;

#[tokio::main]
async fn main() {
    let mut config = utils::load_config().unwrap();
    let account_address = config.pop().unwrap();

    let mut games: Arc<Mutex<Vec<event_parser::Game>>> = Arc::new(Mutex::new(vec![]));

    // TODO: Wrap it with async function for tokio use, maybe with infinite loop
    // Yeah, and don't forget to cut event request on peaces, cause it's looks terrible



    /*let res = tokio::try_join!(utils::build_rocket().launch());
    println!("Error occurs! {}", res.err().unwrap())*/
}
