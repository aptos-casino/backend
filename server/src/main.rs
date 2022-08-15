use crate::aptos_rpc::*;
use std::sync::{Arc, Mutex};

mod aptos_rpc;
mod event_parser;
mod loops;
mod utils;

type GameVecMutex = Arc<Mutex<Vec<event_parser::Game>>>;

#[tokio::main]
async fn main() {
    let mut config = utils::load_config().unwrap();
    let account_address = config.pop().unwrap();

    let mut games: GameVecMutex = Arc::new(Mutex::new(vec![]));

    let private_key_contract = hex::decode(
        String::from("db1ae665378f30cd06c7671a510e1c1fd70de94fbbb692b1d7771bcc8e55551c")
            .as_bytes()
            .as_ref(),
    )
    .unwrap();
    let contract_account = Account::new(Some(private_key_contract));
    let contract_address = contract_account.address();

    let private_key_owner = hex::decode(
        String::from("91f59f5e8213b27326373f25faf92b6e9cfa538f9dd6cac841ccf19d5e89b3ce")
            .as_bytes()
            .as_ref(),
    )
    .unwrap();
    let mut owner_account = Account::new(Some(private_key_owner));
    let account_address = owner_account.address();

    let event_handle_struct = format!(
        "0x{}::CustomContract::MessageHolder/message_change_events",
        contract_address
    );

    // TODO: Wrap it with async function for tokio use, maybe with infinite loop
    // Yeah, and don't forget to cut event request on peaces, cause it's looks terrible

let amount: u64 = 88;
    let payload = serde_json::json!({
        "type": "script_function_payload",
        "function": format!("0x{}::Casino::set_backend_seed", contract_address),
        "type_arguments": [],
        "arguments": [amount.to_string(), format!("{:x}", amount)]
    });
    let res = tokio::task::spawn_blocking(move || {
        let client = RestClient::new(DEVNET.to_string());
        let txn_hash = client.execution_transaction_with_payload(&mut owner_account, payload);
        client.wait_for_transaction(&*txn_hash)
    })
    .await;

    let res = tokio::try_join!(loops::event_parsing_loop(&contract_address, &mut games));
    println!("Error occurs! {}", res.err().unwrap());
}
