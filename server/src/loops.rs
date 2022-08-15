use std::{thread, time::Duration};
use crate::{event_parser, aptos_rpc, GameVecMutex};

pub async fn event_parsing_loop(account_address: &String, games: &mut GameVecMutex) -> Result<(), &'static str>
{
    let mut latest_event_id: u64 = 0;

    loop {
        let event_handle_struct = String::from("0x60dc5deb0b1e9324e831960489be61bde019292ab96de022ae6217565358e94f::CustomContract::MessageHolder/message_change_events");
        let event = aptos_rpc::get_event_by_handle(account_address.clone(), event_handle_struct).await; // Leave it for future
        let res = event_parser::parse_aptos_event(&event, games, &mut latest_event_id);

        if !res.is_empty()
        {
            // TODO: Call contract if all good after event parse and think on how we get new events (games)
            println!("New vector is...{}", res.len());
        }

        thread::sleep(Duration::from_millis(1000));
    }
}