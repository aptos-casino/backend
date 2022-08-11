use rocket::get;

#[get("/health")]
pub fn health() -> String {
    "Working well".to_string()
}

// pub fn give_
