use sha256::digest;

pub fn get_sha256(value: &u64) -> String {
    digest(value.to_string())
}