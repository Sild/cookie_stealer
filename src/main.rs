extern crate os_type;
extern crate dirs;
extern crate cookie_stealer;

use std::borrow::Borrow;
use std::path::{Path, PathBuf};


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

    use cookie_stealer::*;
    let key = cipher::get_cipher_key(user_password.borrow(), os_params.iterations);

    let mut cookies = db::extract_cookies(os_params.cookie_path.as_ref().unwrap().as_path());
    println!("cookies count: {}", cookies.len());
    for c in &mut cookies {
        c.value = cipher::decrypt_value(c.encrypted_value.borrow(), key.borrow());
//        println!("{:?}", c);

    }

}
