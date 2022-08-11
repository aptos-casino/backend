
use http_client::HttpClient;
use http_types::{Method, Request};

use serde_json::json;

const DEVNET: &str = "https://fullnode.devnet.aptoslabs.com";

fn load_config() -> Result<Vec<String>, serde_json::Error> {
    use std::io::Read;
    let mut file = std::fs::File::open("config.json").expect("Can`t open file!");
    let mut some_buf = String::new();
    file.read_to_string(&mut some_buf)
        .expect("Can`t read file!");
    serde_json::from_str(&some_buf)
}

async fn call(account_address: String, handle: String) -> String {
    let req = Request::new(
        Method::Get,
        format!("{}/accounts/{}/events/{}", DEVNET, account_address, handle).as_str(),
    );
    let client = http_client::h1::H1Client::new();
    let res = client.send(req).await.unwrap().body_string().await.unwrap();
    json!(res).to_string()
}

#[tokio::main]
async fn main() {
    let mut config = load_config().unwrap();
    let account_address = config.pop().unwrap();
    let event_handle_struct = String::from("0x60dc5deb0b1e9324e831960489be61bde019292ab96de022ae6217565358e94f::CustomContract::MessageHolder/message_change_events");
    println!("Result: {}", call(account_address, event_handle_struct).await);
}
