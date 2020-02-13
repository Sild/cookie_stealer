extern crate os_type;
extern crate dirs;
extern crate sqlite;
extern crate ring;
extern crate aes;

use ring::{digest, pbkdf2};
use std::path::{Path, PathBuf};
use std::borrow::Borrow;
use std::num::NonZeroU32;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
pub type Key = [u8; 16];



use aes::block_cipher_trait::generic_array::GenericArray;
use aes::block_cipher_trait::BlockCipher;
use aes::Aes128;

#[derive(Debug)]
struct Cookie {
    host: String,
    name: String,
    value: String,
    encrypted_value: String,
}

struct OSSpecificParams {
    cookie_path: Option<PathBuf>,
    iterations: usize,
    password_required: bool,
}

fn get_os_params() -> OSSpecificParams {

    let mut s_params = OSSpecificParams {
        cookie_path: dirs::home_dir(),
        iterations: 0,
        password_required: false,
    };

    match os_type::current_platform().os_type {
        os_type::OSType::OSX => {
            s_params.cookie_path.as_mut().unwrap().push(Path::new("Library/Aplication Support/Google/Chrome/Default/Cookies"));
            s_params.iterations = 1003;
            s_params.password_required = true;
        },
        os_type::OSType::Unknown => {
            s_params.cookie_path = None

        },
        _ => {
            s_params.cookie_path.as_mut().unwrap().push(Path::new(".config/google-chrome/Default/Cookies"));
            s_params.iterations = 1;
            s_params.password_required = false;
        },
    };
    return s_params;
}


fn read_user_input(a_input_msg: &str) -> String {
    use std::io::{stdin,stdout,Write};
    let mut raw=String::new();
    print!("{}", a_input_msg);
    let _=stdout().flush();
    stdin().read_line(&mut raw).expect("Did not enter a correct string");
    if let Some('\n')=raw.chars().next_back() {
        raw.pop();
    }
    if let Some('\r')=raw.chars().next_back() {
        raw.pop();
    }
    return raw;
}

fn extract_cookies(a_path: &std::path::Path) -> Vec<Cookie> {
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

fn get_cipher_key(a_password: & String, a_iterations: usize) -> Key {
    let mut salt = Vec::with_capacity(9);
    salt.extend(String::from("saltysalt").as_bytes());
    let length = 16;

    let mut res : Key = [0u8; 16];;
    pbkdf2::derive(PBKDF2_ALG, NonZeroU32::new(a_iterations as u32).unwrap() , &salt, a_password.as_bytes(), &mut res);

    return res;
//    return String::new();
}

fn decrypt_value(encrypted: &String, key: &Key) -> String {
    let k = GenericArray::from_slice(key.);
    let cipher = Aes128::new(&k);

    let mut block = GenericArray::from_slice(encrypted.as_bytes());
    let mut bl2 = block.clone();
    cipher.decrypt_block(&mut bl2);
    return String::new();
}

fn main() {
    let mut os_params = get_os_params();

    while !(os_params.cookie_path.is_some() && os_params.cookie_path.as_ref().unwrap().is_file() ){
        println!("Fail to find cookie file by path: {}", &os_params.cookie_path.unwrap_or_default().to_string_lossy());
        os_params.cookie_path = Some(PathBuf::from(read_user_input("Input path manually: ")));
    }
    println!("Cookie file found: {}", os_params.cookie_path.as_ref().unwrap().to_string_lossy());

    let user_password = match os_params.password_required {
        true => read_user_input("enter the pass: "),
        false => String::from("peanuts"),
    };
    println!("user password: {}", user_password);

    let key = get_cipher_key(user_password.borrow(), os_params.iterations);

    let mut cookies = extract_cookies(os_params.cookie_path.as_ref().unwrap().as_path());
    println!("cookies count: {}", cookies.len());
    for c in &mut cookies {
        c.value = decrypt_value(c.encrypted_value.borrow(), key.borrow());
//        println!("{:?}", c);
    }

}
