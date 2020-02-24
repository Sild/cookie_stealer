#[derive(Debug)]
pub struct Cookie {
    host: String,
    name: String,
    pub value: String,
    pub encrypted_value: String,
}

pub mod db;
pub mod cipher;