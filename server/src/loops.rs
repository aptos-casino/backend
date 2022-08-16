use crate::{aptos_rpc::*, event_parser, utils, GameVecMutex};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

pub async fn event_parsing_loop(games: &mut GameVecMutex) -> Result<(), &'static str> {
    // This is latest sequence id of event that has unique game id
    let mut latest_event_id: u64 = 0;

    let private_key_contract = hex::decode(
        utils::load_config()
            .unwrap()
            .pop()
            .unwrap()
            .as_bytes()
            .as_ref(),
    )
    .unwrap();
    let contract_account = Arc::new(Mutex::new(Account::new(Some(private_key_contract))));
    let account_address = contract_account.lock().unwrap().address();

    loop {
        let event_handle_struct = String::from("0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::EventsStore/inited_backend_seed_event");
        let mut event = get_event_by_handle(account_address.clone(), event_handle_struct).await; // Leave it for future
        let res = event_parser::parse_aptos_event(&mut event, games, &mut latest_event_id);

        if !res.is_empty() {
            // TODO: Call contract if all good after event parse and think on how we get new events (games)
            println!(
                "New vector is...{}, latest seq: {}",
                res.len(),
                latest_event_id
            );
            for game in res {
                let payload = get_payload(games, &account_address, &game);
                let account_from = contract_account.clone();
                let _ = tokio::task::spawn_blocking(move || {
                    let client = RestClient::new(DEVNET.to_string());
                    let txn_hash = client.execution_transaction_with_payload(
                        &mut account_from.lock().unwrap(),
                        payload,
                    );
                    client.wait_for_transaction(&*txn_hash)
                })
                .await
                .unwrap();
            }
        }

        thread::sleep(Duration::from_millis(1000));
    }
}

pub fn get_payload(
    games: &mut GameVecMutex,
    contract_address: &String,
    game: &event_parser::Game,
) -> serde_json::Value {
    //if there is no such game in the vector, the current event is not a 'game start'
    if games
        .lock()
        .unwrap()
        .iter()
        .any(|some_game| some_game.id == game.id)
    {
        serde_json::json!({
            "type": "script_function_payload",
            "function": format!("0x{}::Casino::set_backend_seed", contract_address),
            "type_arguments": [],
            "arguments": [game.id.to_string(), utils::get_sha256(game.seeds)]
        })
    } else {
        serde_json::json!({
            "type": "script_function_payload",
            "function": format!("0x{}::Casino::set_backend_seed_hash", contract_address),
            "type_arguments": [],
            "arguments": [game.id.to_string(), utils::get_sha256(game.seeds)]
        })
    }
}
