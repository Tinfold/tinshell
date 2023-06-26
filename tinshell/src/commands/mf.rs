use std::fs;
use std::path::Path;
use Shlex;

pub fn mf(mut args: Shlex) {
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
