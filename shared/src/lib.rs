use std::env;

pub fn set_dir(new_dir: &str) {
    if let Err(e) = env::set_current_dir(new_dir) {
        eprintln!("{}", e);
    }
}

pub fn get_dir() -> String {
    let pbuf = std::env::current_dir();
    match pbuf {
        Ok(v) => return v.as_path().to_str().unwrap().to_string(),
        Err(e) => {
            println!("{}", e.to_string());
            return "".to_string();
        }
    }
}
