mod aptos_rpc;
mod utils;
mod event_parser;

#[tokio::main]
async fn main() {
    let mut config = utils::load_config().unwrap();
    let account_address = config.pop().unwrap();

    let mut games: Vec<event_parser::Game> = Vec::new();

    // TODO: Wrap it with async function for tokio use, maybe with infinite loop
    // Yeah, and don't forget to cut event request on peaces, cause it's looks terrible
    let event_handle_struct = String::from("0x60dc5deb0b1e9324e831960489be61bde019292ab96de022ae6217565358e94f::CustomContract::MessageHolder/message_change_events");
    let event = aptos_rpc::get_event_by_handle(account_address, event_handle_struct).await; // Leave it for future
    event_parser::parse_aptos_event(&event, &mut games);

    let res = tokio::try_join!(utils::build_rocket().launch());
    println!("Error occurs! {}", res.err().unwrap())
}
