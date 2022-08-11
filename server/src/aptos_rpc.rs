use http_client::HttpClient;
use http_types::{Method, Request};

use serde_json::json;

const DEVNET: &str = "https://fullnode.devnet.aptoslabs.com";

pub(crate) async fn get_event_by_handle(account_address: String, handle: String) -> serde_json::Value {
    let req = Request::new(
        Method::Get,
        format!("{}/accounts/{}/events/{}", DEVNET, account_address, handle).as_str(),
    );
    let client = http_client::h1::H1Client::new();
    let res = client.send(req).await.unwrap().body_string().await.unwrap();
    json!(res)
}
