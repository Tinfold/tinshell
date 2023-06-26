use std::env;
use std::env::Args;
use std::fs;
use std::path::Path;

pub fn mf(mut args: Args) {
    args.next();
    for temp in args {
        let temps = temp.as_str();
        let path = Path::new(temps);
        match fs::write(&path, "") {
            Ok(_v) => {
                println!(
                    "{}",
                    "Successfully created file ".to_string() + path.to_str().unwrap()
                );
            }
            Err(e) => {
                println!("{}", e)
            }
        };
    }
}

fn main() {
    mf(env::args());
}
