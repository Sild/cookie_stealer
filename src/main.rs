extern crate os_type;
extern crate dirs;
extern crate sqlite;
extern crate crypto;

use std::path::{Path, PathBuf};

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
}

fn get_os_params() -> OSSpecificParams {

    let mut s_params = OSSpecificParams {
        cookie_path: dirs::home_dir(),
        iterations: 0
    };

    match os_type::current_platform().os_type {
        os_type::OSType::OSX => {
            s_params.cookie_path.as_mut().unwrap().push(Path::new("Library/Aplication Support/Google/Chrome/Default/Cookies"));
            s_params.iterations = 32;
        },
        os_type::OSType::Unknown => {
            s_params.cookie_path = None

        },
        _ => {
            s_params.cookie_path.as_mut().unwrap().push(Path::new("SomeLinuxPath"));
            s_params.iterations = 1;
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

fn get_cipher_key(a_password: &mut str, a_iterations: usize) -> String {
    let mut res = [0u8; 16];
    return String::new();
//    crypto::pbkdf2::pbkdf2(&mut a_password.as_bytes(), "saltysalt".as_bytes(), a_iterations as u32, &mut res);
//    return unsafe { String::from_raw_parts(res.as_mut_ptr(), 16, 16) };
}

fn decrypt_cookie(a_cookie: &mut Cookie) {

}

fn main() {
    //let user_password = read_pass();
    //println!("Entered password: {}", user_password);

    let mut os_params = get_os_params();

    while !(os_params.cookie_path.is_some() && os_params.cookie_path.as_ref().unwrap().is_file() ){
        println!("Fail to find cookie file by path: {}", &os_params.cookie_path.unwrap_or_default().to_string_lossy());
        os_params.cookie_path = Some(PathBuf::from(read_user_input("Input path manually: ")));
    }
    println!("Cookie file found: {}", os_params.cookie_path.as_ref().unwrap().to_string_lossy());

    let ket = get_cipher_key(read_user_input("enter the pass: ").as_mut_str(), os_params.iterations);

    let mut cookies = extract_cookies(os_params.cookie_path.as_ref().unwrap().as_path());
    for c in &mut cookies {
        decrypt_cookie(c);
//        println!("{:?}", c);
    }

}
