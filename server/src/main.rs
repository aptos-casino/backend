
use crate::aptos_rpc::*;
//mod aptos_rpc;
mod aptos_rpc;
mod event_parser;
mod utils;

#[tokio::main]
async fn main() {
   /*  let mut config = utils::load_config().unwrap();
    let account_address = config.pop().unwrap();
 */
    let mut games: Vec<event_parser::Game> = Vec::new();

    let private_key_contract = hex::decode(String::from("d04e8327908f079450e0821a0aea077401541bbfab37ea2e5acdd052bdcd77fe").as_bytes().as_ref()).unwrap();
    let contract_account = Account::new(Some(private_key_contract));
    let contract_address = contract_account.address();
    
    let private_key_owner = hex::decode(String::from("2fa7d887f550f879bf89d7bfbb997be11aec337e78eeaa2854ea00dd2b088aa2").as_bytes().as_ref()).unwrap();
    let mut owner_account = Account::new(Some(private_key_owner));
    let account_address = owner_account.address();

    let event_handle_struct = format!("0x{}::CustomContract::MessageHolder/message_change_events", contract_address);
    
    // TODO: Wrap it with async function for tokio use, maybe with infinite loop
    // Yeah, and don't forget to cut event request on peaces, cause it's looks terrible
    let event = aptos_rpc::get_event_by_handle(account_address.clone(), event_handle_struct.clone()).await; // Leave it for future
    println!("{:?}", event);
    //event_parser::parse_aptos_event(&event, &mut games);
    let amount:u8 = 42;
    let payload = serde_json::json!({
        "type": "script_function_payload",
        "function": "0x1::CustonContract::custom_call",
        "type_arguments": [],
        "arguments": [amount.to_string()]
    });
    let res = tokio::task::spawn_blocking(move || {
        let client = RestClient::new(DEVNET.to_string());
        client.execution_transaction_with_payload(&mut owner_account, payload)
    }).await;
    println!("hash {:?}\n", res);
    let event = aptos_rpc::get_event_by_handle(account_address, event_handle_struct).await; // Leave it for future
    println!("{:?}", event);
    // let res = tokio::try_join!(utils::build_rocket().launch());
    // println!("Error occurs! {}", res.err().unwrap())
}
