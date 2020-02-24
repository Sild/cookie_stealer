extern crate sqlite;

use super::Cookie;

pub fn extract_cookies(a_path: &std::path::Path) -> Vec<Cookie> {
    let connection = sqlite::open(a_path).unwrap();

    let mut statement = connection
        .prepare("SELECT host_key, name, encrypted_value FROM cookies")
        .unwrap();

    let mut res: Vec<Cookie> = Vec::new();

    while let sqlite::State::Row = statement.next().unwrap() {
        res.push(Cookie{
            host: statement.read::<String>(0).unwrap(),
            name: statement.read::<String>(1).unwrap(),
            value: String::new(),
            encrypted_value: statement.read::<String>(2).unwrap(),
        });
    };
    return res;
}