use rocket::Rocket;

mod rpc_endpoints;

pub(crate) fn load_config() -> Result<Vec<String>, serde_json::Error> {
    use std::io::Read;
    let mut file = std::fs::File::open("config.json").expect("Can`t open file!");
    let mut some_buf = String::new();
    file.read_to_string(&mut some_buf)
        .expect("Can`t read file!");
    serde_json::from_str(&some_buf)
}

pub(crate) fn build_rocket() -> Rocket<rocket::Build> {
    rocket::build().mount("/v1", rocket::routes![rpc_endpoints::health])
}
