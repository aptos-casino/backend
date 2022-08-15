use crate::{aptos_rpc, event_parser, GameVecMutex};
use std::{thread, time::Duration};

pub async fn event_parsing_loop(
    account_address: &String,
    games: &mut GameVecMutex,
) -> Result<(), &'static str> {
    let mut latest_event_id: u64 = 0;

    loop {
        let event_handle_struct = String::from("0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::EventsStore/inited_backend_seed_event");
        let event =
            aptos_rpc::get_event_by_handle(account_address.clone(), event_handle_struct).await; // Leave it for future
        let res = event_parser::parse_aptos_event(&event, games, &mut latest_event_id);

        if !res.is_empty() {
            // TODO: Call contract if all good after event parse and think on how we get new events (games)
            println!("New vector is...{}, latest seq: {}", res.len(), latest_event_id);
        }

        thread::sleep(Duration::from_millis(1000));
    }
}
