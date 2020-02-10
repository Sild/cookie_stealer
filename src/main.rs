extern crate os_type;
extern crate dirs;

use std::process;
use std::path::Path;


fn get_cookie_default_path() -> Option<String> {

    let home_dir = match dirs::home_dir() {
        Some(path) => match path.to_str() {
            Some(p) => p.to_owned(),
            None => return None,
        },
        None => return None,
    };
    let os_rel_path = match os_type::current_platform().os_type {
        os_type::OSType::OSX => home_dir.to_owned() + "/Library/Application Support/Google/Chrome/Default/Cookies",
        os_type::OSType::Unknown => {
            return None
        }
        _ => "linux default path".to_string(),
    };
    return Some(os_rel_path);
}


fn read_user_input(input_msg: String) -> String {
    use std::io::{stdin,stdout,Write};
    let mut raw=String::new();
    print!("{}", input_msg);
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
    //let user_password = read_pass();
    //println!("Entered password: {}", user_password);

    let cookie_file = match get_cookie_default_path() {
        Some(p) => Path::new(&p.to_owned()),
        None => Path::new(&String::from_string(read_user_input("Fail to get cookie path automatically. Please input it manually: ".to_string()))),
    };

    if !cookie_file.exists() {
        println!("Fail to find cookie file: {}", cookie_file.to_string_lossy());
        process::exit(1);
    }

    println!("File founded: {}", cookie_file.to_string_lossy());

}
