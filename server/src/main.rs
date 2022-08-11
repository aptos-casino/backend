use http_client::HttpClient;
use http_types::{Method, Request};

use serde_json::json;

async fn call() -> String {
    let mut req = Request::new(Method::Get, "https://fullnode.devnet.aptoslabs.com/accounts/891e9c5dfbfa48033bb4802ef80a83ce1b8cf40f310068729ee551c6f8826e35/events/0x60dc5deb0b1e9324e831960489be61bde019292ab96de022ae6217565358e94f::CustomContract::MessageHolder/message_change_events");

    let client = http_client::h1::H1Client::new();
    let res = client.send(req).await.unwrap().body_string().await.unwrap();
    json!(res).to_string()
}

#[tokio::main]
async fn main() {
    println!("Result: {}", call().await);
}
